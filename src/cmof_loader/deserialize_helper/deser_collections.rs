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
use std::marker::PhantomData;

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __BTreeMap__, from array or single object, various Object type tolerant
/// Not 'Option' tolerant, use 'default' for this
pub fn deser_btreemap_using_name_as_key<'de: 'te, 'te: 'de, D, V>(
    deserializer: D,
) -> Result<BTreeMap<String, V>, D::Error>
where
    D: de::Deserializer<'de>,
    V: de::Deserialize<'te>,
    V: XMIIdentity,
{
    struct OneOrVec<String, V>(PhantomData<BTreeMap<String, V>>);

    impl<'de: 'te, 'te: 'de, V: de::Deserialize<'te> + XMIIdentity> de::Visitor<'de>
        for OneOrVec<String, V>
    {
        type Value = BTreeMap<String, V>;

        // Requested type description, returned in error case
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("object or array of object, with \"name\" attribute")
        }

        // Result for Object
        fn visit_map<E>(self, map: E) -> Result<Self::Value, E::Error>
        where
            E: de::MapAccess<'de>,
        {
            let v: V = de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            let k = v.get_xmi_id().get_object_id();
            Ok(BTreeMap::from([(k, v)]))
        }

        // Result for Array
        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut r: BTreeMap<String, V> = BTreeMap::new();
            let big_v: Vec<V> =
                de::Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))?;
            for n in big_v {
                r.insert(n.get_xmi_id().get_object_id(), n);
            }
            Ok(r)
        }
    }

    deserializer.deserialize_any(OneOrVec(PhantomData))
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmof_loader::tests::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[derive(Clone, Debug, PartialEq, Deserialize)]
    struct RandomStruct {
        #[serde(deserialize_with = "deser_btreemap_using_name_as_key")]
        value: BTreeMap<String, SecondRandomStruct>,
    }

    #[derive(Clone, Debug, Deserialize, XMIIdentity)]
    struct SecondRandomStruct {
        #[serde(deserialize_with = "deser_local_xmi_id")]
        #[serde(rename = "_xmi:id")]
        xmi_id: XMIIdLocalReference,
    }

    #[test]
    fn deser_btreemap_using_name_as_key_01_creation() {
        initialize_log_for_test();

        let input_str = r#"{"value": {"_xmi:id": "key_1"}}"#;
        let mut btree_map = BTreeMap::new();
        btree_map.insert(
            "key_1".to_string(),
            SecondRandomStruct {
                xmi_id: XMIIdLocalReference::new_local("key_1".to_string()),
            },
        );
        let value_target = RandomStruct { value: btree_map };
        check_deser_make_no_error(input_str, &value_target);

        let input_str = r#"{"value": [{"_xmi:id": "key_1"}, {"_xmi:id": "key_2"}]}"#;
        let mut btree_map = BTreeMap::new();
        btree_map.insert(
            "key_1".to_string(),
            SecondRandomStruct {
                xmi_id: XMIIdLocalReference::new_local("key_1".to_string()),
            },
        );
        btree_map.insert(
            "key_2  ".to_string(),
            SecondRandomStruct {
                xmi_id: XMIIdLocalReference::new_local("key_2".to_string()),
            },
        );
        let value_target = RandomStruct { value: btree_map };
        check_deser_make_no_error(input_str, &value_target);
    }

    #[test]
    fn deser_btreemap_using_name_as_key_02_check_error() {
        initialize_log_for_test();

        let input_str = r#"{"value": "key_1"}}"#;
        let error_target = "invalid type: string \"key_1\", expected object";
        check_deser_make_error::<RandomStruct>(input_str, error_target);

        let input_str = r#"{"value": 1}}"#;
        let error_target = "invalid type: integer `1`, expected object";
        check_deser_make_error::<RandomStruct>(input_str, error_target);

        let input_str = r#"{"value": 1.0}}"#;
        let error_target = "invalid type: floating point `1.0`, expected object";
        check_deser_make_error::<RandomStruct>(input_str, error_target);

        let input_str = r#"{"value": true}}"#;
        let error_target = "invalid type: boolean `true`, expected object";
        check_deser_make_error::<RandomStruct>(input_str, error_target);
    }
}
