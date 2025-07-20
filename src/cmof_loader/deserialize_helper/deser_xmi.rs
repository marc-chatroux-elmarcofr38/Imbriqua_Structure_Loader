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
