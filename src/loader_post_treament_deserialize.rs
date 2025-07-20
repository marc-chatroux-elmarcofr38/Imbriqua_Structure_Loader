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
#![doc = include_str!("../doc/loader_deserialise_helper.md")]

// Package section
use crate::cmof_loader::*;
use crate::custom_log_tools::*;
use crate::loader_naming_trait::*;

// Dependencies section
use convert_case::{Case, Casing};
pub use infinitable::Infinitable as UnlimitedNatural;
use serde::de;
use std::fmt;
use std::rc::Rc;

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
            formatter.write_str("object or array of object")
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
            r.make_post_deserialize(&mut dict);

            // Return
            Ok(r)
        }
    }

    deserializer.deserialize_any(CustomVisitor {})
}

// fn provide_naming(object: &mut EnumOwnedMember, package_name: &str) {
//     let package_name_no_case = package_name;
//     let package_name_snake_case = package_name.to_case(Case::Snake);
//     let package_name_snake_case = package_name_snake_case.as_str();
//     match object {
//         EnumOwnedMember::Association(ref mut c) => {
//             let class_upper_case = c.name.to_case(Case::UpperCamel);
//             let class_upper_case = class_upper_case.as_str();
//             let class_snake_case = c.name.to_case(Case::Snake);
//             let class_snake_case = class_snake_case.as_str();

//             let mut technical_name = String::from("");
//             technical_name.push_str(package_name_no_case);
//             technical_name.push_str(".cmof#");
//             technical_name.push_str(c.name.as_str());
//             c.technical_name = technical_name;

//             let mut table_name = String::from("");
//             table_name.push_str(package_name_snake_case);
//             table_name.push_str("_");
//             table_name.push_str(class_snake_case);
//             c.table_name = table_name;

//             c.model_name = String::from(class_upper_case);

//             let mut full_name = String::from("");
//             full_name.push_str(package_name_snake_case);
//             full_name.push_str("_association_");
//             full_name.push_str(class_snake_case);
//             c.full_name = full_name;
//         }
//         EnumOwnedMember::Class(ref mut c) => {
//             c.post_deserialize(&String::from(package_name));
//         }
//         EnumOwnedMember::DataType(ref mut c) => {
//             let class_upper_case = c.name.to_case(Case::UpperCamel);
//             let class_upper_case = class_upper_case.as_str();
//             let class_snake_case = c.name.to_case(Case::Snake);
//             let class_snake_case = class_snake_case.as_str();

//             let mut technical_name = String::from("");
//             technical_name.push_str(package_name_no_case);
//             technical_name.push_str(".cmof#");
//             technical_name.push_str(c.name.as_str());
//             c.technical_name = technical_name;

//             let mut table_name = String::from("");
//             table_name.push_str(package_name_snake_case);
//             table_name.push_str("_");
//             table_name.push_str(class_snake_case);
//             c.table_name = table_name;

//             c.model_name = String::from(class_upper_case);

//             let mut full_name = String::from("");
//             full_name.push_str(package_name_snake_case);
//             full_name.push_str("_datatype_");
//             full_name.push_str(class_snake_case);
//             c.full_name = full_name;
//         }
//         EnumOwnedMember::Enumeration(ref mut c) => {
//             let class_upper_case = c.name.to_case(Case::UpperCamel);
//             let class_upper_case = class_upper_case.as_str();
//             let class_snake_case = c.name.to_case(Case::Snake);
//             let class_snake_case = class_snake_case.as_str();

//             let mut technical_name = String::from("");
//             technical_name.push_str(package_name_no_case);
//             technical_name.push_str(".cmof#");
//             technical_name.push_str(c.name.as_str());
//             c.technical_name = technical_name;

//             let mut table_name = String::from("");
//             table_name.push_str(package_name_snake_case);
//             table_name.push_str("_");
//             table_name.push_str(class_snake_case);
//             c.table_name = table_name;

//             c.model_name = String::from(class_upper_case);

//             let mut full_name = String::from("");
//             full_name.push_str(package_name_snake_case);
//             full_name.push_str("_enumeration_");
//             full_name.push_str(class_snake_case);
//             c.full_name = full_name;
//         }
//         EnumOwnedMember::PrimitiveType(ref mut c) => {
//             let class_upper_case = c.name.to_case(Case::UpperCamel);
//             let class_upper_case = class_upper_case.as_str();
//             let class_snake_case = c.name.to_case(Case::Snake);
//             let class_snake_case = class_snake_case.as_str();

//             let mut technical_name = String::from("");
//             technical_name.push_str(package_name_no_case);
//             technical_name.push_str(".cmof#");
//             technical_name.push_str(c.name.as_str());
//             c.technical_name = technical_name;

//             let mut table_name = String::from("");
//             table_name.push_str(package_name_snake_case);
//             table_name.push_str("_");
//             table_name.push_str(class_snake_case);
//             c.table_name = table_name;

//             c.model_name = String::from(class_upper_case);

//             let mut full_name = String::from("");
//             full_name.push_str(package_name_snake_case);
//             full_name.push_str("_primitive_");
//             full_name.push_str(class_snake_case);
//             c.full_name = full_name;
//         }
//     }
// }
