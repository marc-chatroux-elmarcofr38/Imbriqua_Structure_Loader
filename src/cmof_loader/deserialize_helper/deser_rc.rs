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
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __BTreeMap__, from array or single object, various Object type tolerant
/// Not 'Option' tolerant, use 'default' for this
pub fn deser_rc<'de: 'te, 'te: 'de, D, V>(deserializer: D) -> Result<Rc<V>, D::Error>
where
    D: de::Deserializer<'de>,
    V: de::Deserialize<'te>,
{
    struct OneOrOne<V>(PhantomData<Rc<V>>);

    impl<'de: 'te, 'te: 'de, V: de::Deserialize<'te>> de::Visitor<'de> for OneOrOne<V> {
        type Value = Rc<V>;

        // Requested type description, returned in error case
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("object only, for Rc")
        }

        // Result for Object
        fn visit_map<E>(self, map: E) -> Result<Self::Value, E::Error>
        where
            E: de::MapAccess<'de>,
        {
            let v: V = de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            let v = Rc::new(v);
            Ok(v)
        }
    }

    deserializer.deserialize_any(OneOrOne(PhantomData))
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmof_loader::tests::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn deser_rc_01_check_value() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_rc")]
            value: Rc<SecondRandomStruct>,
        }

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct SecondRandomStruct {
            #[serde(deserialize_with = "deser_boolean")]
            value: bool,
        }

        let target_value = RandomStruct {
            value: Rc::new(SecondRandomStruct { value: true }),
        };

        check_deser_make_no_error::<RandomStruct>(r#"{"value": {"value": "true"}}"#, &target_value);
    }

    #[test]
    fn deser_rc_02_check_error() {
        initialize_log_for_test();

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct RandomStruct {
            #[serde(deserialize_with = "deser_rc")]
            value: Rc<SecondRandomStruct>,
        }

        #[derive(Clone, Debug, PartialEq, Deserialize)]
        struct SecondRandomStruct {
            #[serde(deserialize_with = "deser_boolean")]
            value: bool,
        }

        let error_target = "invalid type: string";
        check_deser_make_error::<RandomStruct>(r#"{"value": "true"}"#, error_target);

        let error_target = "invalid type: integer";
        check_deser_make_error::<RandomStruct>(r#"{"value": 1}"#, error_target);

        let error_target = "invalid type: float";
        check_deser_make_error::<RandomStruct>(r#"{"value": 1.0}"#, error_target);

        let error_target = "invalid type: sequence";
        check_deser_make_error::<RandomStruct>(r#"{"value": [1.0, 1.0]}"#, error_target);
    }
}
