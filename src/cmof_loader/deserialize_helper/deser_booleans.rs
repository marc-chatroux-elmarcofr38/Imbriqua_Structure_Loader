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
pub fn deser_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        // Boolean as boolean
        Value::Bool(b) => b,
        // String, True if "yes" or "true"
        Value::String(s) => (s == "yes") || (s == "true"),
        // Number, True if not zero
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? != 0,
        // Null, always False
        Value::Null => false,
        // others
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    })
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __boolean__, return always "true"
pub fn deser_boolean_always_true<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = match de::Deserialize::deserialize(deserializer)? {
        // Boolean as boolean
        Value::Bool(b) => b,
        // String, True if "yes" or "true"
        Value::String(s) => (s == "yes") || (s == "true"),
        // Number, True if not zero
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? != 0,
        // Null, always False
        Value::Null => false,
        // others
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    };
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

/// Deserialising to __boolean__, return always "true"
pub fn deser_boolean_always_false<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let r = match de::Deserialize::deserialize(deserializer)? {
        // Boolean as boolean
        Value::Bool(b) => b,
        // String, True if "yes" or "true"
        Value::String(s) => (s == "yes") || (s == "true"),
        // Number, True if not zero
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? != 0,
        // Null, always False
        Value::Null => false,
        // others
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    };
    Ok(match r {
        true => {
            return Err(de::Error::custom(
                "Wrong boolean check, expected \"false\" only",
            ))
        }
        false => false,
    })
}
