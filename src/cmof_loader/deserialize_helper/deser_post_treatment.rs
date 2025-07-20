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
use std::collections::BTreeMap;
use std::fmt;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __isize__, from string (integer)
pub fn deser_post_treatement_cmof_package<'de: 'te, 'te: 'de, D>(
    deserializer: D,
) -> Result<CMOFPackage, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct CustomVisitor;

    impl<'de: 'te, 'te: 'de> de::Visitor<'de> for CustomVisitor {
        type Value = CMOFPackage;

        // Requested type description, returned in error case
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("object requested (no empty, no list)")
        }

        // Result for Object
        fn visit_map<E>(self, map: E) -> Result<Self::Value, E::Error>
        where
            E: de::MapAccess<'de>,
        {
            // Raw result
            let mut r: Self::Value =
                de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;

            let mut dict = BTreeMap::new();
            let result = r.make_post_deserialize(&mut dict);
            if result.is_err() {
                panic!("{:?}", result.err());
            }

            // Return
            Ok(r)
        }
    }

    deserializer.deserialize_any(CustomVisitor {})
}
