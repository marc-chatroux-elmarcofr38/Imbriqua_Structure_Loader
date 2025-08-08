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

/// Deserialising to __boolean__, from boolean, 'yes' string, 'true' string, number !=0 and Null
/// Allow :
///     - Bool : true and false
///     - String : true for 'yes' and 'true', false for 'no' and 'false', error as default
///     - Number : true for 1, false for 0, error as default()
pub fn deser_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Boolean as boolean
        Value::Bool(b) => b,
        // String, True if "yes" or "true"
        Value::String(s) => {
            if (s == "yes") || (s == "true") {
                true
            } else if (s == "no") || (s == "false") {
                false
            } else {
                return Err(de::Error::custom(format!(
                    "Counln't convert String to bool : {}",
                    s
                )));
            }
        }
        // Number, True if not zero
        Value::Number(num) => {
            let n = num.as_i64().ok_or(de::Error::custom("Invalid number"))?;
            if n == 1 {
                true
            } else if n == 0 {
                false
            } else {
                return Err(de::Error::custom(format!(
                    "Counln't convert Number to bool : {}",
                    n
                )));
            }
        }
        // others
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __boolean__, return always "true", and Error if input isn't "true" equivalent
/// Allow : same that using deser_boolean
pub fn deser_boolean_always_true<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = deser_boolean(deserializer)?;
    Ok(match r {
        true => true,
        false => {
            return Err(de::Error::custom(
                "Wrong boolean check, expected \"true\" only",
            ))
        }
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __boolean__, return always "false", and Error if input isn't "false" equivalent
/// Allow : same that using deser_boolean
pub fn deser_boolean_always_false<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = deser_boolean(deserializer)?;
    Ok(match r {
        true => {
            return Err(de::Error::custom(
                "Wrong boolean check, expected \"false\" only",
            ))
        }
        false => false,
    })
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    fn check_make_error<'de, T>(input_str: &'de str, error_target: &str)
    where
        T: Deserialize<'de> + std::fmt::Debug,
    {
        let r: Result<T, serde_json::Error> = serde_json::from_slice(input_str.as_bytes());
        assert!(r.is_err());

        // Serde error is longer, because adding error source location
        let n = error_target
            .len()
            .min(format!("{}", r.as_ref().unwrap_err()).len());
        assert_eq!(
            format!("{}", r.unwrap_err())[0..n],
            String::from(error_target)
        );
    }

    fn check_make_no_error<'de, T>(input_str: &'de str, value_target: &T)
    where
        T: Deserialize<'de> + std::fmt::Debug + PartialEq,
    {
        let r: Result<T, serde_json::Error> = serde_json::from_slice(input_str.as_bytes());

        if r.is_err() {
            error!("{}", r.as_ref().unwrap_err());
        }

        assert!(r.is_ok());

        assert_eq!(&r.unwrap(), value_target);
    }

    #[test]
    fn test_01_check_true_deser_boolean() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean")]
            value: bool,
        }

        let target_value = RandomStruct { value: true };

        check_make_no_error::<RandomStruct>(r#"{"value": true}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "yes"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "true"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": 1}"#, &target_value);
    }

    #[test]
    fn test_02_check_false_deser_boolean() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean")]
            value: bool,
        }

        let target_value = RandomStruct { value: false };

        check_make_no_error::<RandomStruct>(r#"{"value": false}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "no"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "false"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": 0}"#, &target_value);
    }

    #[test]
    fn test_03_check_others_values_deser_boolean() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean")]
            _value: bool,
        }

        let error_target = "Counln't convert String to bool : test";
        check_make_error::<RandomStruct>(r#"{"_value": "test"}"#, error_target);

        let error_target = "Counln't convert Number to bool : 2";
        check_make_error::<RandomStruct>(r#"{"_value": 2}"#, error_target);
    }

    #[test]
    fn test_04_check_true_deser_boolean_always_true() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean_always_true")]
            value: bool,
        }

        let target_value = RandomStruct { value: true };

        check_make_no_error::<RandomStruct>(r#"{"value": true}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "yes"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "true"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": 1}"#, &target_value);
    }

    #[test]
    fn test_05_check_false_deser_boolean_always_true() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean_always_true")]
            _value: bool,
        }

        let error_target = "Wrong boolean check, expected \"true\" only";

        check_make_error::<RandomStruct>(r#"{"_value": false}"#, error_target);
        check_make_error::<RandomStruct>(r#"{"_value": "no"}"#, error_target);
        check_make_error::<RandomStruct>(r#"{"_value": "false"}"#, error_target);
        check_make_error::<RandomStruct>(r#"{"_value": 0}"#, error_target);
    }

    #[test]
    fn test_06_check_others_values_deser_boolean_always_true() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean_always_true")]
            _value: bool,
        }

        let error_target = "Counln't convert String to bool : test";
        check_make_error::<RandomStruct>(r#"{"_value": "test"}"#, error_target);

        let error_target = "Counln't convert Number to bool : 2";
        check_make_error::<RandomStruct>(r#"{"_value": 2}"#, error_target);
    }

    #[test]
    fn test_07_check_false_deser_boolean_always_false() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean_always_false")]
            value: bool,
        }

        let target_value = RandomStruct { value: false };

        check_make_no_error::<RandomStruct>(r#"{"value": false}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "no"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": "false"}"#, &target_value);
        check_make_no_error::<RandomStruct>(r#"{"value": 0}"#, &target_value);
    }

    #[test]
    fn test_08_check_true_deser_boolean_always_false() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean_always_false")]
            _value: bool,
        }

        let error_target = "Wrong boolean check, expected \"false\" only";

        check_make_error::<RandomStruct>(r#"{"_value": true}"#, error_target);
        check_make_error::<RandomStruct>(r#"{"_value": "yes"}"#, error_target);
        check_make_error::<RandomStruct>(r#"{"_value": "true"}"#, error_target);
        check_make_error::<RandomStruct>(r#"{"_value": 1}"#, error_target);
    }

    #[test]
    fn test_09_check_others_values_deser_boolean_always_false() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_boolean_always_false")]
            _value: bool,
        }

        let error_target = "Counln't convert String to bool : test";
        check_make_error::<RandomStruct>(r#"{"_value": "test"}"#, error_target);

        let error_target = "Counln't convert Number to bool : 2";
        check_make_error::<RandomStruct>(r#"{"_value": 2}"#, error_target);
    }
}
