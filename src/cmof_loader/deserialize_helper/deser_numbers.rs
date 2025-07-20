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

// Dependencies section
pub use infinitable::Infinitable as UnlimitedNatural;
use serde::de;
use serde_json::Value;

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __isize__, from string (integer)
pub fn deser_integer<'de, D>(deserializer: D) -> Result<isize, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // String, True if "yes"
        Value::String(s) => {
            let result = s.parse::<isize>();
            if let Ok(result) = result {
                result
            } else {
                return Err(de::Error::custom("Unknow string : Integer"));
            }
        }
        // others
        _ => return Err(de::Error::custom("Wrong type, expected Integer")),
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __UnlimitedNatural__, from string ("*" or integer)
pub fn deser_unlimited_natural<'de, D>(deserializer: D) -> Result<UnlimitedNatural<usize>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // String, True if "yes"
        Value::String(s) => {
            let result = s.parse::<usize>();
            if s == "*" {
                UnlimitedNatural::Infinity
            } else if let Ok(result) = result {
                UnlimitedNatural::Finite(result)
            } else {
                return Err(de::Error::custom("Unknow string : Integer or \"*\""));
            }
        }
        // others
        _ => return Err(de::Error::custom("Wrong type, expected Integer")),
    })
}
