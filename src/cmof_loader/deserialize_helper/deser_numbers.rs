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

/// Deserialising to __isize__ (lower bound)
/// Allow :
///     - String input of positive integer (up to i32 max value)
pub fn deser_lower_bound<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let result = s.parse::<i32>();
            if let Ok(result) = result {
                result
            } else {
                return Err(de::Error::custom(format!("Unknow string {} : Integer", s)));
            }
        }
        // others
        _ => return Err(de::Error::custom("Wrong type, expected Integer")),
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __UnlimitedNatural__ (upper bound)
/// /// Allow :
///     - String input of positive integer (up to i32 max value)
///     - String input of Infinity, represented as '*'
pub fn deser_upper_bound<'de, D>(deserializer: D) -> Result<UnlimitedNatural<i32>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let result = s.parse::<i32>();
            if s == "*" {
                UnlimitedNatural::Infinity
            } else if let Ok(result) = result {
                UnlimitedNatural::Finite(result)
            } else {
                return Err(de::Error::custom(format!("Unknow string {} : Integer", s)));
            }
        }
        // others
        _ => return Err(de::Error::custom("Wrong type, expected Integer")),
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
    fn test_01_check_value_deser_lower_bound() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_lower_bound")]
            value: i32,
        }

        let target_value = RandomStruct { value: 0 };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "0"}"#, &target_value);

        let target_value = RandomStruct { value: 1 };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "1"}"#, &target_value);

        let target_value = RandomStruct { value: 2 };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "2"}"#, &target_value);

        let target_value = RandomStruct { value: 100 };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "100"}"#, &target_value);
    }

    #[test]
    fn test_02_check_error_value_deser_lower_bound() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_lower_bound")]
            value: i32,
        }

        let error_target = "Unknow string a : Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": "a"}"#, &error_target);

        let error_target = "Unknow string 1.0 : Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": "1.0""}"#, &error_target);

        let error_target = "Wrong type, expected Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_03_check_value_deser_upper_bound() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_upper_bound")]
            value: UnlimitedNatural<i32>,
        }

        let target_value = RandomStruct {
            value: infinitable::Infinity,
        };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "*"}"#, &target_value);

        let target_value = RandomStruct {
            value: infinitable::Finite(0),
        };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "0"}"#, &target_value);

        let target_value = RandomStruct {
            value: infinitable::Finite(1),
        };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "1"}"#, &target_value);

        let target_value = RandomStruct {
            value: infinitable::Finite(2),
        };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "2"}"#, &target_value);

        let target_value = RandomStruct {
            value: infinitable::Finite(100),
        };
        check_deser_make_no_error::<RandomStruct>(r#"{"value": "100"}"#, &target_value);
    }

    #[test]
    fn test_04_check_error_value_deser_upper_bound() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_upper_bound")]
            value: UnlimitedNatural<i32>,
        }

        let error_target = "Unknow string a : Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": "a"}"#, &error_target);

        let error_target = "Unknow string 1.0 : Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": "1.0""}"#, &error_target);

        let error_target = "Wrong type, expected Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": 1.0"}"#, &error_target);

        let error_target = "Wrong type, expected Integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": true"}"#, &error_target);
    }
}
