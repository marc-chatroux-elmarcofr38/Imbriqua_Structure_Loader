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
use crate::custom_log_tools::*;
use crate::loader_cmof_structure::*;
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

            // Add naming on EnumOwnedMember
            let package_name = r.get_name_field();
            let package_name_no_case = package_name.as_str();
            let package_name_snake_case = package_name.to_case(Case::Snake);
            let package_name_snake_case = package_name_snake_case.as_str();
            r.lowercase_name = String::from(package_name_snake_case);
            for (_, p) in &mut r.owned_member {
                let p_unwrap = Rc::get_mut(p).unwrap();
                provide_naming(p_unwrap, package_name_no_case, package_name_snake_case);
            }

            // Complete XMI_ID
            let package_id = package_name;
            r.set_xmi_id_reference(&package_id);

            // Return
            Ok(r)
        }
    }

    deserializer.deserialize_any(CustomVisitor {})
}

fn provide_naming(
    object: &mut EnumOwnedMember,
    package_name_no_case: &str,
    package_name_snake_case: &str,
) {
    match object {
        EnumOwnedMember::Association(ref mut c) => {
            let class_upper_case = c.name.to_case(Case::UpperCamel);
            let class_upper_case = class_upper_case.as_str();
            let class_snake_case = c.name.to_case(Case::Snake);
            let class_snake_case = class_snake_case.as_str();

            let mut technical_name = String::from("");
            technical_name.push_str(package_name_no_case);
            technical_name.push_str(".cmof#");
            technical_name.push_str(c.name.as_str());
            c.technical_name = technical_name;

            let mut table_name = String::from("");
            table_name.push_str(package_name_snake_case);
            table_name.push_str("_");
            table_name.push_str(class_snake_case);
            c.table_name = table_name;

            c.model_name = String::from(class_upper_case);

            let mut full_name = String::from("");
            full_name.push_str(package_name_snake_case);
            full_name.push_str("_association_");
            full_name.push_str(class_snake_case);
            c.full_name = full_name;
        }
        EnumOwnedMember::Class(ref mut c) => {
            let class_upper_case = c.name.to_case(Case::UpperCamel);
            let class_upper_case = class_upper_case.as_str();
            let class_snake_case = c.name.to_case(Case::Snake);
            let class_snake_case = class_snake_case.as_str();

            let mut technical_name = String::from("");
            technical_name.push_str(package_name_no_case);
            technical_name.push_str(".cmof#");
            technical_name.push_str(c.name.as_str());
            c.technical_name = technical_name;

            let mut table_name = String::from("");
            table_name.push_str(package_name_snake_case);
            table_name.push_str("_");
            table_name.push_str(class_snake_case);
            c.table_name = table_name;

            c.model_name = String::from(class_upper_case);

            let mut full_name = String::from("");
            full_name.push_str(package_name_snake_case);
            full_name.push_str("_class_");
            full_name.push_str(class_snake_case);
            c.full_name = full_name;
        }
        EnumOwnedMember::DataType(ref mut c) => {
            let class_upper_case = c.name.to_case(Case::UpperCamel);
            let class_upper_case = class_upper_case.as_str();
            let class_snake_case = c.name.to_case(Case::Snake);
            let class_snake_case = class_snake_case.as_str();

            let mut technical_name = String::from("");
            technical_name.push_str(package_name_no_case);
            technical_name.push_str(".cmof#");
            technical_name.push_str(c.name.as_str());
            c.technical_name = technical_name;

            let mut table_name = String::from("");
            table_name.push_str(package_name_snake_case);
            table_name.push_str("_");
            table_name.push_str(class_snake_case);
            c.table_name = table_name;

            c.model_name = String::from(class_upper_case);

            let mut full_name = String::from("");
            full_name.push_str(package_name_snake_case);
            full_name.push_str("_datatype_");
            full_name.push_str(class_snake_case);
            c.full_name = full_name;
        }
        EnumOwnedMember::Enumeration(ref mut c) => {
            let class_upper_case = c.name.to_case(Case::UpperCamel);
            let class_upper_case = class_upper_case.as_str();
            let class_snake_case = c.name.to_case(Case::Snake);
            let class_snake_case = class_snake_case.as_str();

            let mut technical_name = String::from("");
            technical_name.push_str(package_name_no_case);
            technical_name.push_str(".cmof#");
            technical_name.push_str(c.name.as_str());
            c.technical_name = technical_name;

            let mut table_name = String::from("");
            table_name.push_str(package_name_snake_case);
            table_name.push_str("_");
            table_name.push_str(class_snake_case);
            c.table_name = table_name;

            c.model_name = String::from(class_upper_case);

            let mut full_name = String::from("");
            full_name.push_str(package_name_snake_case);
            full_name.push_str("_enumeration_");
            full_name.push_str(class_snake_case);
            c.full_name = full_name;
        }
        EnumOwnedMember::PrimitiveType(ref mut c) => {
            let class_upper_case = c.name.to_case(Case::UpperCamel);
            let class_upper_case = class_upper_case.as_str();
            let class_snake_case = c.name.to_case(Case::Snake);
            let class_snake_case = class_snake_case.as_str();

            let mut technical_name = String::from("");
            technical_name.push_str(package_name_no_case);
            technical_name.push_str(".cmof#");
            technical_name.push_str(c.name.as_str());
            c.technical_name = technical_name;

            let mut table_name = String::from("");
            table_name.push_str(package_name_snake_case);
            table_name.push_str("_");
            table_name.push_str(class_snake_case);
            c.table_name = table_name;

            c.model_name = String::from(class_upper_case);

            let mut full_name = String::from("");
            full_name.push_str(package_name_snake_case);
            full_name.push_str("_primitive_");
            full_name.push_str(class_snake_case);
            c.full_name = full_name;
        }
    }
}

/// ahahahaah
pub trait SetXMIIdReference {
    /// ahahaha
    fn set_xmi_id_reference(&mut self, package_id: &String);
}

impl SetXMIIdReference for CMOFAssociation {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
        for (_, p) in &mut self.owned_end {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumOwnedEnd::Property(ref mut c) => c.set_xmi_id_reference(package_id),
            }
        }
    }
}

impl SetXMIIdReference for CMOFClass {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
        for (_, p) in &mut self.owned_attribute {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumOwnedAttribute::Property(ref mut c) => c.set_xmi_id_reference(package_id),
            }
        }
        for (_, p) in &mut self.owned_rule {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumOwnedRule::Constraint(ref mut c) => c.set_xmi_id_reference(package_id),
            }
        }
    }
}

impl SetXMIIdReference for CMOFConstraint {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
        match self.specification {
            EnumSpecification::OpaqueExpression(ref mut c) => c.set_xmi_id_reference(package_id),
        }
    }
}

impl SetXMIIdReference for CMOFDataType {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
        for (_, p) in &mut self.owned_attribute {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumOwnedAttribute::Property(ref mut c) => c.set_xmi_id_reference(package_id),
            }
        }
        for (_, p) in &mut self.owned_rule {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumOwnedRule::Constraint(ref mut c) => c.set_xmi_id_reference(package_id),
            }
        }
    }
}

impl SetXMIIdReference for CMOFEnumeration {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
        for (_, p) in &mut self.owned_attribute {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumOwnedLiteral::EnumerationLiteral(ref mut c) => {
                    c.set_xmi_id_reference(package_id)
                }
            }
        }
    }
}

impl SetXMIIdReference for CMOFEnumerationLiteral {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
    }
}

impl SetXMIIdReference for CMOFOpaqueExpression {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
    }
}

impl SetXMIIdReference for CMOFPackage {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
        for (_, p) in &mut self.package_import {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumPackageImport::PackageImport(ref mut c) => c.set_xmi_id_reference(package_id),
            }
        }
        for (_, p) in &mut self.owned_member {
            let p_unwrap = Rc::get_mut(p).unwrap();
            match p_unwrap {
                EnumOwnedMember::Association(ref mut c) => c.set_xmi_id_reference(package_id),
                EnumOwnedMember::Class(ref mut c) => c.set_xmi_id_reference(package_id),
                EnumOwnedMember::DataType(ref mut c) => c.set_xmi_id_reference(package_id),
                EnumOwnedMember::Enumeration(ref mut c) => c.set_xmi_id_reference(package_id),
                EnumOwnedMember::PrimitiveType(ref mut c) => c.set_xmi_id_reference(package_id),
            }
        }
    }
}

impl SetXMIIdReference for CMOFPackageImport {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
    }
}

impl SetXMIIdReference for CMOFPrimitiveType {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
    }
}

impl SetXMIIdReference for CMOFProperty {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
    }
}

impl SetXMIIdReference for CMOFTag {
    fn set_xmi_id_reference(&mut self, package_id: &String) {
        self.xmi_id.set_package(&package_id);
    }
}
