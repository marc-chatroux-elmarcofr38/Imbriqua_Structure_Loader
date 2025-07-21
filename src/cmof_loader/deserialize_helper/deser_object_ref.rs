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

/// Convert string with space to vec of string, splitting on space
pub fn deser_local_xmi_id<'de, D>(deserializer: D) -> Result<XMIIdLocalReference, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Split text
        Value::String(s) => match s.find(".cmof#") {
            Some(split_index) => {
                let a = s[split_index..].replace(".cmof#", "").to_string();
                let b = s[..split_index].to_string();
                XMIIdLocalReference::new_global(a, b)
            }
            None => XMIIdLocalReference::new_local(s),
        },
        _ => {
            return Err(de::Error::custom(
                "Wrong type, expected String for converting to XMI ID Reference",
            ))
        }
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string with space to vec of string, splitting on space
pub fn deser_xmi_id<'de, D>(deserializer: D) -> Result<XMIIdReference, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Split text
        Value::String(s) => match s.find(".cmof#") {
            Some(split_index) => {
                let a = s[split_index..].replace(".cmof#", "").to_string();
                let b = s[..split_index].to_string();
                XMIIdReference::new_global(a, b)
            }
            None => XMIIdReference::new_local(s),
        },
        _ => {
            return Err(de::Error::custom(
                "Wrong type, expected String for converting to XMI ID Reference",
            ))
        }
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Convert string with space to vec of string, splitting on space
pub fn deser_href<'de, D>(deserializer: D) -> Result<XMIIdReference, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Split text
        Value::String(s) => match s.find(".cmof#") {
            Some(split_index) => {
                let a = s[split_index..].replace(".cmof#", "").to_string();
                let b = s[..split_index].to_string();
                XMIIdReference::new_global(a, b)
            }
            None => {
                return Err(de::Error::custom(format!(
                    "HRef deserialize error : no \".cmof#\" for {}",
                    s
                )))
            }
        },
        _ => {
            return Err(de::Error::custom(
                "Wrong type, expected String for converting to HRef Reference",
            ))
        }
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising 2-String Vec, from String, require a 1-whitespace String
pub fn deser_split_2_space_href<'de, D>(
    deserializer: D,
) -> Result<(XMIIdReference, XMIIdReference), D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let content: Vec<&str> = s.split_whitespace().collect();

            // Len criteria
            if content.len() != 2 {
                return Err(de::Error::custom(format!(
                    "Need only one whitespace : raw='{}'",
                    s
                )));
            }

            let r: Vec<String> = content.iter().map(|x| String::from(*x)).collect();
            let object_1 = r[0].clone();
            let object_2 = r[1].clone();
            let ref_1 = XMIIdReference::new_local(object_1);
            let ref_2 = XMIIdReference::new_local(object_2);
            (ref_1, ref_2)
        }
        // Value::Null => vec![String::from("empty")],
        _ => return Err(de::Error::custom("Wrong type, expected String")),
    })
}
