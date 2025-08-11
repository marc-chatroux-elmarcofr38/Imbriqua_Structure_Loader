/*
Copyright 2023-2024 CHATROUX MARC

This file is part of Imbriqua Structure, a interpreter of BPMN model files (in UML notation) for
Imbriqua Engine project

Imbriqua Structure is free software: you can redistribute it and/or modify it under the terms of
the GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

Imbriqua Structure is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE.See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Imbriqua Structure.
If not, see <https://www.gnu.org/licenses/>.
*/

#![warn(dead_code)]
#![warn(missing_docs)]

// Package section
use crate::cmof_loader::*;

// Dependencies section
use serde::de;
use serde_json::Value;

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __String__, from name (prevent suspicious name)
/// Allow :
///     - String, (and rebrand 'type' as 'r#type')
pub fn deser_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        Value::String(s) => match s.as_str() {
            "type" => "r#type".to_string(),
            _ => s,
        },
        // others
        _ => return Err(de::Error::custom("Wrong type, expected string")),
    })
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmof_loader::tests::{check_deser_make_error, check_deser_make_no_error};
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn test_01_check_value_deser_name() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_name")]
            value: String,
        }

        let target_value = RandomStruct {
            value: String::from("name"),
        };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "name"}"#, &target_value);

        let target_value = RandomStruct {
            value: String::from("r#type"),
        };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "type"}"#, &target_value);
    }

    #[test]
    fn test_02_check_error_value_deser_name() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_name")]
            value: String,
        }

        let error_target = "Wrong type, expected string";
        check_deser_make_error::<RandomStruct>(r#"{"value": 1}"#, &error_target);

        let error_target = "Wrong type, expected string";
        check_deser_make_error::<RandomStruct>(r#"{"value": true"}"#, &error_target);

        let error_target = "Wrong type, expected string";
        check_deser_make_error::<RandomStruct>(r#"{"value": 1.0"}"#, &error_target);
    }
}
