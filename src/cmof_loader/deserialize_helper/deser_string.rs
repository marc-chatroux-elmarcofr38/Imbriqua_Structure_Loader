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
pub fn deser_spaced_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Split text
        Value::String(s) => s.split(" ").map(str::to_string).collect(),
        // Null, always False
        Value::Null => Vec::new(),
        _ => return Err(de::Error::custom("Wrong type, expected String")),
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __String__, from name (prevent suspicious name)
pub fn deser_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // String, True if "yes" or "true"
        Value::String(s) => match s.as_str() {
            "type" => "r#type".to_string(),
            _ => s,
        },
        // others
        _ => return Err(de::Error::custom("Wrong type, expected string")),
    })
}
