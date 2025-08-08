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

/// Convert string with space to Vec of XMIIdLocalReference, builded with the content between space
/// Allow :
///     - String, with or without '.cmof#' segment, and with or without space
///     - Empty
///
/// Generalisation of other function
fn deserialiaze_spaced_local_xmi_id<'de, D>(
    deserializer: D,
) -> Result<Vec<XMIIdLocalReference>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Split text
        Value::String(s) => {
            let splited: Vec<String> = s.split(" ").map(str::to_string).collect();
            let mut r: Vec<XMIIdLocalReference> = Vec::new();
            for s in splited {
                if s.len() > 0 {
                    match cut_string_for_xmi_id_local_reference(s) {
                        Ok(v) => {
                            r.push(v);
                        }
                        Err(err) => return Err(de::Error::custom(err)),
                    }
                }
            }
            r
        }
        _ => {
            return Err(de::Error::custom(
                "Wrong type, expected String (deserialiaze_spaced_local_xmi_id)",
            ))
        }
    })
}

fn cut_string_for_xmi_id_local_reference(content: String) -> Result<XMIIdLocalReference, String> {
    match content.find(".cmof#") {
        Some(split_index) => {
            let n = split_index + 6;
            let a = content[n..].to_string();
            let b = content[..split_index].to_string();

            // Check granularity : NOT '.cmof' in both part
            if a.contains(".cmof#") {
                return Err(format!("Granularity error : '{}' containt '.cmof#'", a));
            }
            if b.contains(".cmof#") {
                return Err(format!("Granularity error : '{}' containt '.cmof#'", b));
            }

            Ok(XMIIdLocalReference::new_global(a, b))
        }
        None => Ok(XMIIdLocalReference::new_local(content)),
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string with space to Vec of XMIIdReference, builded with the content between space
/// Allow :
///     - String, with or without '.cmof#' segment, and with or without space
///     - Empty
///
/// Generalisation of other function
fn deserialiaze_spaced_global_xmi_id<'de, D>(
    deserializer: D,
) -> Result<Vec<XMIIdReference<EnumWeakCMOF>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Split text
        Value::String(s) => {
            let splited: Vec<String> = s.split(" ").map(str::to_string).collect();
            let mut r: Vec<XMIIdReference<EnumWeakCMOF>> = Vec::new();
            for s in splited {
                if s.len() > 0 {
                    match cut_string_for_xmi_id_global_reference(s) {
                        Ok(v) => {
                            r.push(v);
                        }
                        Err(err) => return Err(de::Error::custom(err)),
                    }
                }
            }
            r
        }
        _ => {
            return Err(de::Error::custom(
                "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)",
            ))
        }
    })
}

fn cut_string_for_xmi_id_global_reference(
    content: String,
) -> Result<XMIIdReference<EnumWeakCMOF>, String> {
    match content.find(".cmof#") {
        Some(split_index) => {
            let n = split_index + 6;
            let a = content[n..].to_string();
            let b = content[..split_index].to_string();

            // Check granularity : NOT '.cmof' in both part
            if a.contains(".cmof#") {
                return Err(format!("Granularity error : '{}' containt '.cmof#'", a));
            }
            if b.contains(".cmof#") {
                return Err(format!("Granularity error : '{}' containt '.cmof#'", b));
            }

            Ok(XMIIdReference::new_global(a, b))
        }
        None => Ok(XMIIdReference::new_local(content)),
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string to a XMIIdLocalReference
/// Allow :
///     - String, with or without '.cmof#' segment, and without space
pub fn deser_local_xmi_id<'de, D>(deserializer: D) -> Result<XMIIdLocalReference, D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = deserialiaze_spaced_local_xmi_id(deserializer)?;

    // Len criteria
    match r.len() {
        1 => Ok(r[0].clone()),
        _ => Err(de::Error::custom(format!(
            "Need only zero whitespace : raw='{:?}'",
            r
        ))),
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string to a XMIIdReference
/// Allow :
///     - String, with or without '.cmof#' segment, and without space
pub fn deser_xmi_id<'de, D>(deserializer: D) -> Result<XMIIdReference<EnumWeakCMOF>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = deserialiaze_spaced_global_xmi_id(deserializer)?;

    // Len criteria
    match r.len() {
        1 => Ok(r[0].clone()),
        _ => Err(de::Error::custom(format!(
            "Need only zero whitespace : raw='{:?}'",
            r
        ))),
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string with space to Vec of XMIIdReference
/// Allow :
///     - String, with or without '.cmof#' segment, and without space
///     - Empty
pub fn deser_option_xmi_id<'de, D>(
    deserializer: D,
) -> Result<Option<XMIIdReference<EnumWeakCMOF>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = deserialiaze_spaced_global_xmi_id(deserializer)?;

    // Len criteria
    match r.len() {
        0 => Ok(None),
        1 => Ok(Some(r[0].clone())),
        _ => Err(de::Error::custom(format!(
            "Need only zero whitespace (or empty) : raw='{:?}'",
            r
        ))),
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string with space to Vec of XMIIdReference, builded with the content between space
/// Allow :
///     - String, with or without '.cmof#' segment, and with or without space
///     - Empty
pub fn deser_spaced_xmi_id<'de, D>(
    deserializer: D,
) -> Result<Vec<XMIIdReference<EnumWeakCMOF>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    deserialiaze_spaced_global_xmi_id(deserializer)
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string with 1-space to Vec of XMIIdReference, builded with the content between space
/// Allow :
///     - String, with or without '.cmof#' segment, and with ONE space
///     - Empty
pub fn deser_2_space_xmi_id<'de, D>(
    deserializer: D,
) -> Result<(XMIIdReference<EnumWeakCMOF>, XMIIdReference<EnumWeakCMOF>), D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = deserialiaze_spaced_global_xmi_id(deserializer)?;

    // Len criteria
    match r.len() {
        2 => Ok((r[0].clone(), r[1].clone())),
        _ => Err(de::Error::custom(format!(
            "Need only one whitespace : raw='{:?}'",
            r
        ))),
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert Object with '_href' attribute to XMIIdReference
/// Allow :
///     - Object with '_href' String attribute, with or without '.cmof#' segment, and without space
pub fn deser_superclass_object_xmi_id<'de, D>(
    deserializer: D,
) -> Result<Vec<XMIIdReference<EnumWeakCMOF>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        Value::Object(map) => {
            // Need '_href' key
            if !map.contains_key("_href") {
                return Err(de::Error::custom(
                    "Wrong Object content, expected '_href' field (Sting type) for converting to XMIIdReference (deser_superclass_object_xmi_id)",
                ));
            }
            // Catch '_href' value
            let v = map.get("_href").unwrap().clone();
            match v {
                Value::String(s) => match cut_string_for_xmi_id_global_reference(s) {
                    Ok(v) => Vec::from([v]),
                    Err(err) => return Err(de::Error::custom(err)),
                },
                _ => {
                    return Err(de::Error::custom(
                    "Wrong Object content, expected '_href' String type field for converting to XMIIdReference (deser_superclass_object_xmi_id)",
                    ))
                }
            }
        }
        _ => {
            return Err(de::Error::custom(
                    "Wrong content, expected Object with '_href' field (Sting type) for converting to XMIIdReference (deser_superclass_object_xmi_id)",
            ))
        }
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
    fn test_01_check_value_deserialiaze_spaced_local_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deserialiaze_spaced_local_xmi_id")]
            value: Vec<XMIIdLocalReference>,
        }

        let target_value = RandomStruct {
            value: vec![
                XMIIdLocalReference::new_global(
                    String::from("object_1"),
                    String::from("package_1"),
                ),
                XMIIdLocalReference::new_global(
                    String::from("object_2"),
                    String::from("package_2"),
                ),
                XMIIdLocalReference::new_local(String::from("object_3")),
                XMIIdLocalReference::new_global(
                    String::from("object_4"),
                    String::from("package_4"),
                ),
                XMIIdLocalReference::new_local(String::from("object_5")),
            ],
        };

        let input_str = r#"{"value": "package_1.cmof#object_1 package_2.cmof#object_2 object_3 package_4.cmof#object_4 object_5"}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let target_value = RandomStruct { value: vec![] };

        let input_str = r#"{"value": ""}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);
    }

    #[test]
    fn test_02_check_error_deserialiaze_spaced_local_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deserialiaze_spaced_local_xmi_id")]
            value: Vec<XMIIdLocalReference>,
        }

        let error_target = "Granularity error : 'b.cmof#c' containt '.cmof#'";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b.cmof#c"}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_local_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_local_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_03_check_value_deserialiaze_spaced_global_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deserialiaze_spaced_global_xmi_id")]
            value: Vec<XMIIdReference<EnumWeakCMOF>>,
        }

        let target_value = RandomStruct {
            value: vec![
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1")),
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2")),
                XMIIdReference::new_local(String::from("object_3")),
                XMIIdReference::new_global(String::from("object_4"), String::from("package_4")),
                XMIIdReference::new_local(String::from("object_5")),
            ],
        };

        let input_str = r#"{"value": "package_1.cmof#object_1 package_2.cmof#object_2 object_3 package_4.cmof#object_4 object_5"}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let target_value = RandomStruct { value: vec![] };

        let input_str = r#"{"value": ""}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);
    }

    #[test]
    fn test_04_check_error_deserialiaze_spaced_global_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deserialiaze_spaced_global_xmi_id")]
            value: Vec<XMIIdReference<EnumWeakCMOF>>,
        }

        let error_target = "Granularity error : 'b.cmof#c' containt '.cmof#'";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b.cmof#c"}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_05_check_deser_local_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_local_xmi_id")]
            value: XMIIdLocalReference,
        }

        let target_value = RandomStruct {
            value: XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            ),
        };

        let input_str = r#"{"value": "package_1.cmof#object_1"}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let error_target = "Need only zero whitespace : raw='[Complete XMIIdLocalReference RefCell of 'a-b', Complete XMIIdLocalReference RefCell of 'c-d']";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b c.cmof#d"}"#, &error_target);

        let error_target = "Need only zero whitespace : raw='[]'";
        check_make_error::<RandomStruct>(r#"{"value": ""}"#, &error_target);

        let error_target = "Granularity error : 'b.cmof#c' containt '.cmof#'";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b.cmof#c"}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_local_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_local_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_06_check_deser_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_xmi_id")]
            value: XMIIdReference<EnumWeakCMOF>,
        }

        let target_value = RandomStruct {
            value: XMIIdReference::new_global(String::from("object_1"), String::from("package_1")),
        };

        let input_str = r#"{"value": "package_1.cmof#object_1"}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let error_target = "Need only zero whitespace : raw='[UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'a-b', UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'c-d']";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b c.cmof#d"}"#, &error_target);

        let error_target = "Need only zero whitespace : raw='[]'";
        check_make_error::<RandomStruct>(r#"{"value": ""}"#, &error_target);

        let error_target = "Granularity error : 'b.cmof#c' containt '.cmof#'";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b.cmof#c"}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_07_check_deser_option_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_option_xmi_id")]
            value: Option<XMIIdReference<EnumWeakCMOF>>,
        }

        let target_value = RandomStruct {
            value: Some(XMIIdReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            )),
        };

        let input_str = r#"{"value": "package_1.cmof#object_1"}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let target_value = RandomStruct { value: None };

        let input_str = r#"{"value": ""}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let error_target = "Need only zero whitespace (or empty) : raw='[UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'a-b', UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'c-d']";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b c.cmof#d"}"#, &error_target);

        let error_target = "Granularity error : 'b.cmof#c' containt '.cmof#'";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b.cmof#c"}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_08_check_value_deser_spaced_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_spaced_xmi_id")]
            value: Vec<XMIIdReference<EnumWeakCMOF>>,
        }

        let target_value = RandomStruct {
            value: vec![
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1")),
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2")),
                XMIIdReference::new_local(String::from("object_3")),
                XMIIdReference::new_global(String::from("object_4"), String::from("package_4")),
                XMIIdReference::new_local(String::from("object_5")),
            ],
        };

        let input_str = r#"{"value": "package_1.cmof#object_1 package_2.cmof#object_2 object_3 package_4.cmof#object_4 object_5"}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let target_value = RandomStruct { value: vec![] };

        let input_str = r#"{"value": ""}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);
    }

    #[test]
    fn test_09_check_error_deser_spaced_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_spaced_xmi_id")]
            value: Vec<XMIIdReference<EnumWeakCMOF>>,
        }

        let error_target = "Granularity error : 'b.cmof#c' containt '.cmof#'";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b.cmof#c"}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_10_check_deser_2_space_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_2_space_xmi_id")]
            value: (XMIIdReference<EnumWeakCMOF>, XMIIdReference<EnumWeakCMOF>),
        }

        let target_value = RandomStruct {
            value: (
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1")),
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2")),
            ),
        };

        let input_str = r#"{"value": "package_1.cmof#object_1 package_2.cmof#object_2"}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let error_target = "Need only one whitespace : raw='[]'";
        check_make_error::<RandomStruct>(r#"{"value": ""}"#, &error_target);

        let error_target = "Need only one whitespace : raw='[UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'a-b']";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b"}"#, &error_target);

        let error_target = "Need only one whitespace : raw='[UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'a-b', UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'c-d', UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of 'e-f']";
        check_make_error::<RandomStruct>(
            r#"{"value": "a.cmof#b c.cmof#d e.cmof#f"}"#,
            &error_target,
        );

        let error_target = "Granularity error : 'b.cmof#c' containt '.cmof#'";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b.cmof#c"}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong type, expected String (deserialiaze_spaced_global_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);
    }

    #[test]
    fn test_11_check_deser_superclass_object_xmi_id() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_superclass_object_xmi_id")]
            value: Vec<XMIIdReference<EnumWeakCMOF>>,
        }

        let target_value = RandomStruct {
            value: vec![XMIIdReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            )],
        };

        let input_str = r#"{"value": {"_href" :"package_1.cmof#object_1"}}"#;
        check_make_no_error::<RandomStruct>(input_str, &target_value);

        let input_str =
            r#"{"value": {"_href" :"package_1.cmof#object_1 package_2.cmof#object_2"}}"#;
        let error_target =
            "Granularity error : 'object_1 package_2.cmof#object_2' containt '.cmof#'";
        check_make_error::<RandomStruct>(input_str, &error_target);

        let error_target = "Wrong content, expected Object with '_href' field (Sting type) for converting to XMIIdReference (deser_superclass_object_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": "a.cmof#b"}"#, &error_target);

        let error_target = "Wrong content, expected Object with '_href' field (Sting type) for converting to XMIIdReference (deser_superclass_object_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": 1.0}"#, &error_target);

        let error_target = "Wrong content, expected Object with '_href' field (Sting type) for converting to XMIIdReference (deser_superclass_object_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": true}"#, &error_target);

        let error_target = "Wrong Object content, expected '_href' String type field for converting to XMIIdReference (deser_superclass_object_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": {"_href": 1}}"#, &error_target);

        let error_target = "Wrong Object content, expected '_href' String type field for converting to XMIIdReference (deser_superclass_object_xmi_id)";
        check_make_error::<RandomStruct>(r#"{"value": {"_href":true}}"#, &error_target);

        let input_str = r#"{"value": {"href" :"package_1.cmof#object_1"}}"#;
        let error_target = "Wrong Object content, expected '_href' field (Sting type) for converting to XMIIdReference (deser_superclass_object_xmi_id)";
        check_make_error::<RandomStruct>(input_str, &error_target);
    }
}
