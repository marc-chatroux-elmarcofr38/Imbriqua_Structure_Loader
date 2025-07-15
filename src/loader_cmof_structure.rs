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
#![doc = include_str!("../doc/loader_cmof_structure.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_deserialise_helper::*;

// Dependencies section
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Association Object
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
    /// memberEnd attribute, 2 for CMOF
    #[serde(rename = "_memberEnd")]
    #[serde(deserialize_with = "deser_split_2_space")]
    pub member_end: (String, String),
    /// Optional ownedEnd object
    #[serde(rename = "ownedEnd")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_end: Vec<EnumOwnedEnd>,
    // navigableOwnedEnd forbidden
    /// Optional _isDerived object, need to by "false"
    #[serde(rename = "_isDerived")]
    #[serde(deserialize_with = "deser_boolean_always_false")]
    #[serde(default = "default_false")]
    pub is_derived: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Class Object
pub struct CMOFClass {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// isAbstract attribute
    #[serde(rename = "_isAbstract")]
    #[serde(deserialize_with = "deser_boolean")]
    #[serde(default = "default_false")]
    pub is_abstract: bool,
    /// Optional superClass attribute (simple superClass)
    #[serde(rename = "_superClass")]
    #[serde(deserialize_with = "deser_spaced_string")]
    #[serde(default = "default_empty_vec")]
    pub super_class: Vec<String>,
    /// Optional superClass object (complex superClass)
    #[serde(rename = "superClass")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub super_class_link: Vec<EnumSuperClass>,
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Constraint Object
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
    pub specification: EnumSpecification,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF DataType Object
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Enumeration Object
pub struct CMOFEnumeration {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// Optional ownedLiteral object arry
    #[serde(rename = "ownedLiteral")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_attribute: Vec<EnumOwnedLiteral>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF EnumerationLiteral Object
pub struct CMOFEnumerationLiteral {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// classifier attribute
    #[serde(rename = "_classifier")]
    pub classifier: String,
    /// enumeration attribute
    #[serde(rename = "_enumeration")]
    pub enumeration: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF OpaqueExpression Object
pub struct CMOFOpaqueExpression {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// body attribute
    #[serde(rename = "body")]
    pub body: String,
    /// language attribute
    #[serde(rename = "language")]
    pub language: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Package Object
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
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub package_import: Vec<EnumPackageImport>,
    /// Optional ownedMember object array
    #[serde(rename = "ownedMember")]
    #[serde(deserialize_with = "deser_vec")]
    #[serde(default = "default_empty_vec")]
    pub owned_member: Vec<EnumOwnedMember>,
}

impl CMOFPackage {
    /// Lowercase name of the package (no '.', no '#', no uppercase)
    pub fn get_lowercase_name(&self) -> String {
        let str_result = Path::new(&self.name)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_ascii_lowercase();
        str_result.to_case(Case::Snake)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF PackageImport Object
pub struct CMOFPackageImport {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// importingNamespace attribute
    #[serde(rename = "_importingNamespace")]
    pub importing_namespace: String,
    /// importedPackage object
    #[serde(rename = "importedPackage")]
    pub imported_package: EnumImportedPackage,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF PrimitiveType Object
pub struct CMOFPrimitiveType {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Property Object
pub struct CMOFProperty {
    /// xmi:id attribute
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    /// name attribute
    #[serde(rename = "_name")]
    #[serde(deserialize_with = "deser_name")]
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
    pub complex_type: Option<EnumType>,
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
    pub upper: UnlimitedNatural<usize>,
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
    pub association: Option<String>,
    /// Optional redefinedProperty object
    #[serde(rename = "redefinedProperty")]
    pub redefined_property_link: Option<EnumRedefinedProperty>,
    /// Optional SubsettedProperty object
    #[serde(rename = "subsettedProperty")]
    pub subsetted_property_link: Option<EnumSubsettedProperty>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Tag Object
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing package file
pub struct FilePackage {
    /// cmof:Package object
    #[serde(rename = "cmof:Package")]
    pub package: CMOFPackage,
    /// cmof:Tag object list
    #[serde(rename = "cmof:Tag")]
    pub tags: Vec<CMOFTag>,
    /// xmi version
    #[serde(rename = "_xmi:version")]
    pub xmi_versions: String,
    /// XLM namespace XMI
    #[serde(rename = "_xmlns:xmi")]
    pub xmi_uri: String,
    /// XML namespace CMOF
    #[serde(rename = "_xmlns:cmof")]
    pub cmof_uri: String,
    /// XML namespace
    #[serde(rename = "_xmlns")]
    pub ns: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedMember Tag
pub enum EnumOwnedMember {
    /// OwnedMember with cmof:Association type
    #[serde(rename = "cmof:Association")]
    Association(CMOFAssociation),
    /// OwnedMember with cmof:Class type
    #[serde(rename = "cmof:Class")]
    Class(CMOFClass),
    /// OwnedMember with cmof:Enumeration type
    #[serde(rename = "cmof:Enumeration")]
    Enumeration(CMOFEnumeration),
    /// OwnedMember with cmof:PrimitiveType type
    #[serde(rename = "cmof:PrimitiveType")]
    PrimitiveType(CMOFPrimitiveType),
    /// OwnedMember with cmof:DataType type
    #[serde(rename = "cmof:DataType")]
    DataType(CMOFDataType),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedAttribute Tag
pub enum EnumOwnedAttribute {
    /// OwnedAttribute with cmof:Property type
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedEnd Tag
pub enum EnumOwnedEnd {
    /// OwnedEnd with cmof:Property type
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedRule Tag
pub enum EnumOwnedRule {
    /// OwnedRule with cmof:Constraint type
    #[serde(rename = "cmof:Constraint")]
    Constraint(CMOFConstraint),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedLiteral Tag
pub enum EnumOwnedLiteral {
    /// OwnedLiteral with cmof:EnumerationLiteral type
    #[serde(rename = "cmof:EnumerationLiteral")]
    EnumerationLiteral(CMOFEnumerationLiteral),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing RedefinedProperty Tag
pub enum EnumRedefinedProperty {
    /// RedefinedProperty with cmof:Property type
    #[serde(rename = "cmof:Property")]
    Property(RedefinedProperty),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SubsettedProperty Tag
pub enum EnumSubsettedProperty {
    /// SubsettedProperty with cmof:Property type
    #[serde(rename = "cmof:Property")]
    Property(SubsettedProperty),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing RedefinedProperty object
pub struct RedefinedProperty {
    /// Link to property of RedefinedProperty
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SubsettedProperty object
pub struct SubsettedProperty {
    /// Link to property of SubsettedProperty
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Complex Class link object
pub enum EnumSuperClass {
    /// SuperClass Tag with cmof:Class type
    #[serde(rename = "cmof:Class")]
    Class(SuperClass),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SuperClass Tag
pub struct SuperClass {
    /// Link to Class of SuperClass
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Complex Class link object
pub enum EnumImportedPackage {
    /// ImportedPackage Tag with cmof:Package type
    #[serde(rename = "cmof:Package")]
    ImportedPackage(ImportedPackage),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing ImportedPackage object
pub struct ImportedPackage {
    /// Link of the package
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Specification Tag
pub enum EnumSpecification {
    /// Specification Tag with cmof:OpaqueExpression type
    #[serde(rename = "cmof:OpaqueExpression")]
    OpaqueExpression(CMOFOpaqueExpression),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing PackageImport Tag
pub enum EnumPackageImport {
    /// PackageImport Tag with cmof:PackageImport type
    #[serde(rename = "cmof:PackageImport")]
    PackageImport(CMOFPackageImport),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
// #[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing VisibilityKind type (UML doc)
pub enum EnumVisibilityKind {
    /// UML Public VisibilityKind
    #[serde(rename = "public")]
    Public,
    /// UML Private VisibilityKind
    #[serde(rename = "private")]
    Private,
    /// UML Protected VisibilityKind
    #[serde(rename = "protected")]
    Protected,
    /// UML Package VisibilityKind
    #[serde(rename = "package")]
    Package,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Complex Class link object
pub enum EnumType {
    /// ImportedPackage Tag with cmof:Class type
    #[serde(rename = "cmof:Class")]
    ClassLink(ClassLink),
    /// ImportedPackage Tag with cmof:PrimitiveType type
    #[serde(rename = "cmof:PrimitiveType")]
    PrimitiveTypeLink(PrimitiveTypeLink),
    /// ImportedPackage Tag with cmof:DataType type
    #[serde(rename = "cmof:DataType")]
    DataTypeLink(DataTypeLink),
}

fn cut_href_and_edit(content: &String) -> (String, String, String) {
    match content.find('#') {
        Some(split_index) => {
            let package: String = content[..split_index]
                .to_string()
                .replace(".cmof", "")
                .to_ascii_lowercase()
                .to_case(Case::Snake);
            let split_index = split_index + 1;
            let class: String = content[split_index..].to_string();
            let result = package.clone() + "::" + class.as_str();
            return (package, class, result);
        }
        None => {
            error!(
                "href attribute without '#' separator : href = \"{}\"",
                content
            );
            panic!(
                "href attribute without '#' separator : href = \"{}\"",
                content
            );
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Class link
pub struct ClassLink {
    /// Link of the Class type
    #[serde(rename = "_href")]
    pub href: String,
}

impl ClassLink {
    /// Get the package of href
    pub fn get_package(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.0;
    }
    /// Get the class of href
    pub fn get_class(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.1;
    }
    /// Get the class link of href
    pub fn get_rust_call_link(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.2;
    }
}

impl PrimitiveTypeLink {
    /// Get the package of href
    pub fn get_package(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.0;
    }
    /// Get the class of href
    pub fn get_class(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.1;
    }
    /// Get the class link of href
    pub fn get_rust_call_link(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.2;
    }
}

impl DataTypeLink {
    /// Get the package of href
    pub fn get_package(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.0;
    }
    /// Get the class of href
    pub fn get_class(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.1;
    }
    /// Get the class link of href
    pub fn get_rust_call_link(&self) -> String {
        let result = cut_href_and_edit(&self.href);
        return result.2;
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Primitive Type link
pub struct PrimitiveTypeLink {
    /// Link of the Class type
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Data Type link
pub struct DataTypeLink {
    /// Link of the Class type
    #[serde(rename = "_href")]
    pub href: String,
}
