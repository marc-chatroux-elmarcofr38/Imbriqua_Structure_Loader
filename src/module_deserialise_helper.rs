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

#![warn(missing_docs)]
#![doc = include_str!("../doc/module_deserialise_helper.md")]

// Package section
use crate::module_cmof_conversion::*;
use crate::module_log::*;

// Dependencies section
pub use infinitable::Infinitable as UnlimitedNatural;
use serde::{de, de::MapAccess, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;
use std::marker::PhantomData;

/// Deserialising to __isize__, from string (integer)
pub fn deser_integer<'de, D>(deserializer: D) -> Result<isize, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
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

/// Deserialising to __UnlimitedNatural__, from string ("*" or integer)
pub fn deser_unlimited_natural<'de, D>(deserializer: D) -> Result<UnlimitedNatural<isize>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        // String, True if "yes"
        Value::String(s) => {
            let result = s.parse::<isize>();
            if s == "*" {
                UnlimitedNatural::Infinity
            } else if s == "-*" {
                UnlimitedNatural::NegativeInfinity
            } else if let Ok(result) = result {
                UnlimitedNatural::Finite(result)
            } else {
                return Err(de::Error::custom(
                    "Unknow string : Integer, \"*\" or \"-*\"",
                ));
            }
        }
        // others
        _ => return Err(de::Error::custom("Wrong type, expected Integer")),
    })
}

/// Deserialising to __boolean__, from boolean, 'yes' string, number !=0 and Null
pub fn deser_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        // Boolean as boolean
        Value::Bool(b) => b,
        // String, True if "yes"
        Value::String(s) => s == "yes",
        // Number, True if not zero
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? != 0,
        // Null, always False
        Value::Null => false,
        // others
        _ => return Err(de::Error::custom("Wrong type, expected boolean")),
    })
}

/// Deserialising to __Vec__, from array or single object, various Object type tolerant
/// Not 'Option' tolerant, use 'default' for this
pub fn deser_vec<'de: 'te, 'te: 'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'te>,
{
    struct OneOrVec<T>(PhantomData<Vec<T>>);

    impl<'de: 'te, 'te: 'de, T: Deserialize<'te>> de::Visitor<'de> for OneOrVec<T> {
        type Value = Vec<T>;

        // Requested type description, returned in error case
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("object or array of object")
        }

        // Result for Null
        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(vec![])
        }

        // Result for Object
        fn visit_map<E>(self, map: E) -> Result<Self::Value, E::Error>
        where
            E: MapAccess<'de>,
        {
            Ok(vec![Deserialize::deserialize(
                de::value::MapAccessDeserializer::new(map),
            )?])
        }

        // Result for Array
        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(OneOrVec(PhantomData))
}

/// Deserialising 2-String Vec, from String, require a 1-whitespace String
pub fn deser_split_2_space<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        Value::String(s) => {
            let content: Vec<&str> = s.split_whitespace().collect();

            // Len criteria
            if content.len() != 2 {
                return Err(de::Error::custom(format!(
                    "Need only one whitespace : raw='{}'",
                    s
                )));
            }

            content.iter().map(|x| String::from(*x)).collect()
        }
        // Value::Null => vec![String::from("empty")],
        _ => return Err(de::Error::custom("Wrong type, expected String")),
    })
}

/// __False__, as default value for serde_default
pub fn default_false() -> bool {
    false
}

/// __True__, as default value for serde_default
pub fn default_true() -> bool {
    true
}

/// Empty String, as default value for serde_default
pub fn default_lower() -> isize {
    1
}

/// Empty String, as default value for serde_default
pub fn default_upper() -> UnlimitedNatural<isize> {
    infinitable::Finite(1)
}

/// Empty String, as default value for serde_default
pub fn default_empty_string() -> String {
    String::new()
}

/// Empty Vec, as default value for serde_default
pub fn default_empty_vec<T>() -> Vec<T> {
    Vec::new()
}

/// Default VisibilityKind, as default value for serde_default
pub fn default_visibility() -> EnumVisibilityKind {
    EnumVisibilityKind::Public
}
