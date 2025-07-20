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
use std::rc::Rc;

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __Vec__, from array or single object, various Object type tolerant
/// Not 'Option' tolerant, use 'default' for this
pub fn deser_vec<'de: 'te, 'te: 'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::Deserialize<'te>,
{
    struct OneOrVec<T>(PhantomData<Vec<T>>);

    impl<'de: 'te, 'te: 'de, T: de::Deserialize<'te>> de::Visitor<'de> for OneOrVec<T> {
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
            E: de::MapAccess<'de>,
        {
            Ok(vec![de::Deserialize::deserialize(
                de::value::MapAccessDeserializer::new(map),
            )?])
        }

        // Result for Array
        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            de::Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(OneOrVec(PhantomData))
}

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
    V: GetXMIId,
{
    struct OneOrVec<String, V>(PhantomData<BTreeMap<String, V>>);

    impl<'de: 'te, 'te: 'de, V: de::Deserialize<'te> + GetXMIId> de::Visitor<'de>
        for OneOrVec<String, V>
    {
        type Value = BTreeMap<String, V>;

        // Requested type description, returned in error case
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("object or array of object, with \"name\" attribute")
        }

        // Result for Null
        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(BTreeMap::new())
        }

        // Result for Object
        fn visit_map<E>(self, map: E) -> Result<Self::Value, E::Error>
        where
            E: de::MapAccess<'de>,
        {
            let v: V = de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            let k = v.get_xmi_id_field();
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
                r.insert(n.get_xmi_id_field(), n);
            }
            Ok(r)
        }
    }

    deserializer.deserialize_any(OneOrVec(PhantomData))
}

// ####################################################################################################
//
// ####################################################################################################

/// Deserialising to __BTreeMap__, from array or single object, various Object type tolerant
/// Not 'Option' tolerant, use 'default' for this
pub fn deser_btreemap_with_rc_using_name_as_key<'de: 'te, 'te: 'de, D, V>(
    deserializer: D,
) -> Result<BTreeMap<String, Rc<V>>, D::Error>
where
    D: de::Deserializer<'de>,
    V: de::Deserialize<'te>,
    V: GetXMIId,
{
    struct OneOrVec<String, V>(PhantomData<BTreeMap<String, Rc<V>>>);

    impl<'de: 'te, 'te: 'de, V: de::Deserialize<'te> + GetXMIId> de::Visitor<'de>
        for OneOrVec<String, V>
    {
        type Value = BTreeMap<String, Rc<V>>;

        // Requested type description, returned in error case
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("object or array of object, with \"name\" attribute")
        }

        // Result for Null
        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(BTreeMap::new())
        }

        // Result for Object
        fn visit_map<E>(self, map: E) -> Result<Self::Value, E::Error>
        where
            E: de::MapAccess<'de>,
        {
            let v: V = de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            let v = Rc::new(v);
            let k = v.get_xmi_id_field();
            Ok(BTreeMap::from([(k, v)]))
        }

        // Result for Array
        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            let mut r: BTreeMap<String, Rc<V>> = BTreeMap::new();
            let big_v: Vec<V> =
                de::Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))?;
            for n in big_v {
                let k = n.get_xmi_id_field();
                let v = Rc::new(n);
                r.insert(k, v);
            }
            Ok(r)
        }
    }

    deserializer.deserialize_any(OneOrVec(PhantomData))
}

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
