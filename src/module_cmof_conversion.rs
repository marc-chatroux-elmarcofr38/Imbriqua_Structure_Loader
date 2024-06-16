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
#![doc = include_str!("../doc/module_cmof_conversion.md")]

// Package section
use crate::module_deserialise_helper::*;
use crate::module_file_manager::*;
use crate::module_log::*;
use crate::module_rust_struct_exporter::*;

// Dependencies section
use lazy_static::lazy_static;
use serde::Deserialize;
pub use serde_json;
use std::collections::HashMap;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

// cmof:Association
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF Association
pub struct CMOFAssociation {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// visibility attribute
    #[serde(rename = "_visibility")]
    pub visibility: EnumVisibilityKind,
    /// Optional memberEnd attribute
    #[serde(rename = "_memberEnd")]
    #[serde(deserialize_with = "deser_split_2_space")]
    pub member_end: Vec<String>,
    /// Optional ownedEnd object
    #[serde(rename = "ownedEnd")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_end: Vec<EnumOwnedEnd>,
    /// Optional ownedEnd object
    #[serde(rename = "_isDerived")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_derived: bool,
}

impl WritingSruct for CMOFAssociation {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = writeln!(writer, "// struct_level : {}", self.name);
    }
}

// cmof:Class
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF Class
pub struct CMOFClass {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// isAbstract attribute
    ///
    /// ```text
    /// from : UML 2.5.1
    /// file : formal-17-12-06.pdf
    /// section : 11.8.3.5
    ///
    /// isAbstract : Boolean [1..1] = false
    /// If true, the Class does not provide a complete declaration and cannot be instantiated. An abstract Class is
    /// typically used as a target of Associations or Generalizations.
    /// ```
    #[serde(rename = "_isAbstract")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_abstract: bool,
    /// Optional superClass attribute (simple superClass)
    #[serde(rename = "_superClass")]
    pub super_class: Option<String>,
    /// Optional superClass object (complex superClass)
    #[serde(rename = "superClass")]
    pub super_class_link: Option<EnumSuperClass>,
    /// Optional ownedAttribute object array
    #[serde(rename = "ownedAttribute")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_attribute: Vec<EnumOwnedAttribute>,
    /// Optional ownedRule object
    #[serde(rename = "ownedRule")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_rule: Vec<EnumOwnedRule>,
}

impl WritingSruct for CMOFClass {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (Class : {})",
            self.xmi_id, self.name
        );
        let _ = writeln!(writer, "///");
        let _ = writeln!(writer, "/// ```json");
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("/// "));
        let _ = writeln!(writer, "/// ```");

        // Start of Struct
        let _ = writeln!(writer, "pub struct {} {{", self.name);

        // superClass
        let _ = write!(
            writer,
            "{}",
            format!("super_class : {:#?}", self.super_class).prefix("    // RAW | ")
        );
        let _ = write!(
            writer,
            "{}",
            format!("super_class_link : {:#?}", self.super_class_link).prefix("    // RAW | ")
        );
        if self.super_class.is_some() {
            let contents = self.super_class.as_ref().unwrap();
            for content in contents.split(' ') {
                let _ = writeln!(
                    writer,
                    "    pub heritage_{} : {},",
                    content.to_case(Case::Snake),
                    content
                );
            }
        } else if self.super_class_link.is_some() {
            match self.super_class_link.as_ref().unwrap() {
                EnumSuperClass::Class(content) => {
                    let content = &content.href;

                    //
                    match content.find('#') {
                        Some(split_index) => {
                            let package_file: String = content[..split_index].to_string();
                            let package_file: String = package_file.replace(".cmof", "");
                            let split_index = split_index + 1;
                            let package_class: String = content[split_index..].to_string();
                            let _ = writeln!(
                                writer,
                                "    pub heritage_{} : {}::{},",
                                package_class.to_case(Case::Snake),
                                package_file.to_case(Case::Snake),
                                package_class
                            );
                        }
                        None => {
                            panic!("href without '#' : {}", content)
                        }
                    }
                }
            }
        }

        // End of struct
        let _ = writeln!(writer, "}}");
    }
}

// cmof:Constraint
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF Constrait
pub struct CMOFConstraint {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// constrainedElement attribute
    #[serde(rename = "_constrainedElement")]
    pub constrained_element: String,
    /// namespace attribute
    #[serde(rename = "_namespace")]
    pub namespace: String,
    /// specification object
    #[serde(rename = "specification")]
    pub specification: Specification,
}

impl WritingSruct for CMOFConstraint {
    fn wrt_struct_level(&self, _writer: &mut File) {}
}

// cmof:DataType
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF DataType
pub struct CMOFDataType {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// Optional ownedAttribute object array
    #[serde(rename = "ownedAttribute")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_attribute: Vec<EnumOwnedAttribute>,
    /// Optional ownedRule object
    #[serde(rename = "ownedRule")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_rule: Vec<EnumOwnedRule>,
}

impl WritingSruct for CMOFDataType {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (DataType : {})",
            self.xmi_id, self.name
        );

        // Start of Struct
        let _ = writeln!(writer, "pub struct {} {{", self.name);

        // OwnedAttribute
        for content in self.owned_attribute.iter() {
            content.wrt_struct_level(writer);
        }

        // ownedRule
        for content in self.owned_rule.iter() {
            content.wrt_struct_level(writer);
        }

        // End of struct
        let _ = writeln!(writer, "}}");
    }
}

// cmof:Enumeration
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF Enumeration
pub struct CMOFEnumeration {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    name: String,
    /// Optional ownedLiteral object arry
    #[serde(rename = "ownedLiteral")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    owned_attribute: Vec<EnumOwnedLiteral>,
}

impl WritingSruct for CMOFEnumeration {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (Enumeration : {})",
            self.xmi_id, self.name
        );

        // Enum
        let _ = writeln!(
            writer,
            "pub enum {} {{",
            self.name.to_case(Case::UpperCamel)
        );
        for content in self.owned_attribute.iter() {
            content.wrt_struct_level(writer);
        }
        let _ = writeln!(writer, "}}");
    }
}

// cmof:EnumerationLiteral
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF EnumerationLiteral
pub struct CMOFEnumerationLiteral {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_classifier")]
    classifier: String,
    #[serde(rename = "_enumeration")]
    enumeration: String,
}

impl WritingSruct for CMOFEnumerationLiteral {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "    /// '{}' from (id : '{}', name : '{}')",
            self.name.to_case(Case::UpperCamel),
            self.xmi_id,
            self.name
        );
        let _ = writeln!(writer, "    {}, ", self.name.to_case(Case::UpperCamel));
    }
}

// cmof:OpaqueExpression
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF OpaqueExpression
pub struct CMOFOpaqueExpression {}

// cmof:Package
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF Package
pub struct CMOFPackage {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// uri attribute
    #[serde(rename = "_uri")]
    pub uri: String,
    /// Optional packageImport object array
    #[serde(rename = "packageImport")]
    #[serde(default = "default_empty_vec")]
    pub package_import: Vec<CMOFPackageImport>,
    /// Optional ownedMember object array
    #[serde(rename = "ownedMember")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    owned_member: Vec<EnumOwnedMember>,
}

impl WritingUse for CMOFPackage {
    fn wrt_use_level(&self, writer: &mut File) {
        for import in self.package_import.iter() {
            import.wrt_use_level(writer);
        }
    }
}

impl WritingSruct for CMOFPackage {
    fn wrt_struct_level(&self, writer: &mut File) {
        for class in self.owned_member.iter() {
            class.wrt_struct_level(writer)
        }
    }
}

// cmof:PackageImport
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF PackageImport
pub struct CMOFPackageImport {
    /// xmi:type attribute
    #[serde(rename = "_xmi:type")] //TODO remove it, using Enum
    pub xmi_type: String,
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// importingNamespace attribute
    #[serde(rename = "_importingNamespace")]
    pub importing_namespace: String,
    /// importedPackage object
    #[serde(rename = "importedPackage")]
    pub imported_package: ImportedPackage,
}

impl WritingUse for CMOFPackageImport {
    fn wrt_use_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(writer, "/// Conversion of {} (PackageImport)", self.xmi_id);

        let content = self.imported_package.href.clone();
        let content = content.replace(".cmof#_0", "");
        let content = content.to_case(Case::Snake);
        let _ = writeln!(writer, "use crate::{};", content);
    }
}

// cmof:PrimitiveType
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF PrimitiveType
pub struct CMOFPrimitiveType {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
}

lazy_static! {
    static ref PRIMITIVE_TYPE_LINK: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Boolean", "std::primitive::bool");
        m.insert("Integer", "std::primitive::u64");
        m.insert("Real", "std::primitive::f64");
        m.insert("String", "std::string::String");
        m
    };
}

impl WritingSruct for CMOFPrimitiveType {
    fn wrt_struct_level(&self, writer: &mut File) {
        // Doc
        let _ = writeln!(writer);
        let _ = writeln!(
            writer,
            "/// Conversion of {} (PrimitiveType : {})",
            self.xmi_id, self.name
        );

        // Importing linked struct
        if PRIMITIVE_TYPE_LINK.get(self.name.as_str()).is_some() {
            let content = PRIMITIVE_TYPE_LINK.get(self.name.as_str()).unwrap();
            let _ = writeln!(writer, "pub use {} as {};", content, self.name);
        } else {
            panic!("PANIC : {} isn't in PRIMITIVE_TYPE_LINK", self.name)
        }
    }
}

// cmof:Property
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF Property (from UML)
pub struct CMOFProperty {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// visibility attribute
    #[serde(rename = "_visibility")]
    #[serde(default = "default_visibility")]
    pub visibility: EnumVisibilityKind,
    /// Optional type attribute (simple type)
    #[serde(rename = "_type")]
    pub simple_type: Option<String>,
    /// Optional type object (complex type)
    #[serde(rename = "type")]
    pub complex_type: Option<ComplexType>,
    /// Optional datatype attribute
    #[serde(rename = "_datatype")]
    pub datatype: Option<String>,
    /// Optional lower attribute
    #[serde(rename = "_lower")]
    #[serde(deserialize_with = "deser_integer")]
    #[serde(default = "default_lower")]
    pub lower: isize,
    /// Optional upper attribute
    #[serde(rename = "_upper")]
    #[serde(deserialize_with = "deser_unlimited_natural")]
    #[serde(default = "default_upper")]
    pub upper: UnlimitedNatural<isize>,
    /// Optional default attribute
    #[serde(rename = "_default")]
    pub default: Option<String>,
    /// isReadOnly attribute
    #[serde(rename = "_isReadOnly")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_read_only: bool,
    /// isComposite attribute
    #[serde(rename = "_isComposite")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_composite: bool,
    /// isUnique attribute
    #[serde(rename = "_isUnique")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_unique: bool,
    /// isOrdered attribute
    #[serde(rename = "_isOrdered")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_ordered: bool,
    /// Optional isAbstract attribute
    #[serde(rename = "_isAbstract")]
    pub is_abstract: Option<String>,
    /// isDerived attribute
    #[serde(rename = "_isDerived")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_derived: bool,
    /// isDerivedUnion attribute
    #[serde(rename = "_isDerivedUnion")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_derived_union: bool,
    /// Optional subsettedProperty attribute
    #[serde(rename = "_subsettedProperty")]
    pub subsetted_property: Option<String>,
    /// Optional owningAssociation attribute
    #[serde(rename = "_owningAssociation")]
    #[serde(default = "default_empty_string")]
    pub owning_association: String,
    /// Optional association attribute
    #[serde(rename = "_association")]
    #[serde(default = "default_empty_string")]
    pub association: String,
    /// Optional redefinedProperty object
    #[serde(rename = "redefinedProperty")]
    pub redefined_property_link: Option<EnumRedefinedProperty>,
    /// Optional SubsettedProperty object
    #[serde(rename = "subsettedProperty")]
    pub subsetted_property_link: Option<EnumSubsettedProperty>,
}

impl CMOFProperty {
    fn is_public(&self) -> bool {
        self.visibility == EnumVisibilityKind::Public
    }

    fn is_vec(&self) -> bool {
        self.upper > infinitable::Finite(1)
    }

    fn is_option(&self) -> bool {
        self.lower < 1 && !self.is_vec()
    }

    fn get_type(&self) -> String {
        if self.simple_type.is_some() {
            let content = self.simple_type.as_ref().unwrap();
            content.clone()
        } else if self.complex_type.is_some() {
            self.complex_type.as_ref().unwrap().get_type_name()
        } else {
            String::from("None")
        }
    }
}

impl WritingSruct for CMOFProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        // type
        let name = self.name.to_case(Case::Snake);

        let _ = writeln!(
            writer,
            "    {a} {name} : {b}{c}{content}{d}{e},",
            name = name,
            content = self.get_type(),
            a = if self.is_public() { "pub" } else { "" },
            b = if self.is_option() { "Option<" } else { "" },
            c = if self.is_vec() { "Vec<" } else { "" },
            d = if self.is_vec() { ">" } else { "" },
            e = if self.is_option() { ">" } else { "" }
        );
    }
}

// cmof:Tag
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// CMOF Tag
pub struct CMOFTag {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// value attribute
    #[serde(rename = "_value")]
    pub value: String,
    /// element attribute
    #[serde(rename = "_element")]
    pub element: String,
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FilePackage {
    #[serde(rename = "cmof:Package")]
    pub package: CMOFPackage,
    #[serde(rename = "cmof:Tag")]
    pub tags: Vec<CMOFTag>,
    #[serde(rename = "_xmi:version")]
    pub xmi_versions: String,
    #[serde(rename = "_xmlns:xmi")]
    pub xmi_uri: String,
    #[serde(rename = "_xmlns:cmof")]
    pub cmof_uri: String,
    #[serde(rename = "_xmlns")]
    pub ns: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedMember {
    #[serde(rename = "cmof:Association")]
    Association(CMOFAssociation),
    #[serde(rename = "cmof:Class")]
    Class(CMOFClass),
    #[serde(rename = "cmof:Enumeration")]
    Enumeration(CMOFEnumeration),
    #[serde(rename = "cmof:PrimitiveType")]
    PrimitiveType(CMOFPrimitiveType),
    #[serde(rename = "cmof:DataType")]
    DataType(CMOFDataType),
}

impl WritingSruct for EnumOwnedMember {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedMember::Association(content) => {
                // content.wrt_struct_level(writer); // @marc-chatroux-elmarcofr38
            }
            EnumOwnedMember::Enumeration(content) => {
                content.wrt_struct_level(writer);
            }
            EnumOwnedMember::DataType(content) => {
                content.wrt_struct_level(writer);
            }
            EnumOwnedMember::Class(content) => {
                content.wrt_struct_level(writer);
            }
            EnumOwnedMember::PrimitiveType(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumOwnedAttribute {
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

impl WritingSruct for EnumOwnedAttribute {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedAttribute::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumOwnedEnd {
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

impl WritingSruct for EnumOwnedEnd {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedEnd::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumOwnedRule {
    #[serde(rename = "cmof:Constraint")]
    Constraint(CMOFConstraint),
}

impl WritingSruct for EnumOwnedRule {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedRule::Constraint(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumOwnedLiteral {
    #[serde(rename = "cmof:EnumerationLiteral")]
    EnumerationLiteral(CMOFEnumerationLiteral),
}

impl WritingSruct for EnumOwnedLiteral {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumRedefinedProperty {
    #[serde(rename = "cmof:Property")]
    Property(RedefinedProperty),
}

impl WritingSruct for EnumRedefinedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumRedefinedProperty::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumSubsettedProperty {
    #[serde(rename = "cmof:Property")]
    Property(SubsettedProperty),
}

impl WritingSruct for EnumSubsettedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        match self {
            EnumSubsettedProperty::Property(content) => {
                content.wrt_struct_level(writer);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RedefinedProperty {
    #[serde(rename = "_href")]
    pub href: String,
}

impl WritingSruct for RedefinedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "// struct_level : {} (RedefinedProperty)",
            self.href
        );
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SubsettedProperty {
    #[serde(rename = "_href")]
    pub href: String,
}

impl WritingSruct for SubsettedProperty {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(
            writer,
            "// struct_level : {} (SubsettedProperty)",
            self.href
        );
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumSuperClass {
    #[serde(rename = "cmof:Class")]
    Class(SuperClass),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SuperClass {
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportedPackage {
    #[serde(rename = "_xmi:type")]
    pub r#type: String,
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Specification {
    #[serde(rename = "_xmi:type")]
    pub xmi_type: String,
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "body")]
    pub body: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
// #[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
pub enum EnumVisibilityKind {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "package")]
    Package,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ComplexType {
    #[serde(rename = "_xmi:type")]
    pub xmi_type: String,
    #[serde(rename = "_href")]
    pub href: String,
}

impl ComplexType {
    fn get_type_name(&self) -> String {
        String::from("None")
    }
}

impl WritingSruct for ComplexType {
    fn wrt_struct_level(&self, writer: &mut File) {
        let _ = writeln!(writer, "    // struct_level : {} (ComplexType)", self.href);
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OwnedLiteral {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_log::tests::initialize_log_for_test;

    #[test]
    fn module_elc_____() {
        // Logs
        initialize_log_for_test();
        // Setting
        // Preparing
        // Test
    }
}
