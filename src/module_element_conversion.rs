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
#![doc = include_str!("../doc/module_dependencies_explorer.md")]

// Package section
use crate::module_log::*;

// Dependencies section
// use minidom::Element;
// pub use quick_xml::de::from_str;
// use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
pub use serde_json;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

// cmof:Association
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFAssociation {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_visibility")]
    pub visibility: String,
    #[serde(rename = "_memberEnd")]
    pub member_end: Option<String>,
    #[serde(rename = "ownedEnd")]
    pub owned_end: Option<EnumOwnedEnd>,
}

// cmof:Class
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFClass {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_isAbstract")]
    pub is_abstract: Option<String>,
    #[serde(rename = "_superClass")]
    pub super_class: Option<String>,
    #[serde(rename = "ownedAttribute")]
    pub owned_attribute: Option<Vec<EnumOwnedAttribute>>,
    #[serde(rename = "ownedRule")]
    pub owned_rule: Option<EnumOwnedRule>,
    #[serde(rename = "superClass")]
    pub super_class_link: Option<EnumSuperClass>,
}

// cmof:Constraint
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFConstraint {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_constrainedElement")]
    pub constrained_element: String,
    #[serde(rename = "_namespace")]
    pub namespace: String,
    #[serde(rename = "specification")]
    pub specification: Specification,
}

// cmof:DataType
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFDataType {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "ownedAttribute")]
    pub owned_attribute: Option<Vec<EnumOwnedAttribute>>,
    #[serde(rename = "ownedRule")]
    pub owned_rule: Option<EnumOwnedRule>,
}

// cmof:Enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFEnumeration {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "ownedLiteral")]
    pub owned_attribute: Option<Vec<EnumOwnedLiteral>>,
}

// cmof:EnumerationLiteral
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFEnumerationLiteral {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_classifier")]
    pub classifier: String,
    #[serde(rename = "_enumeration")]
    pub enumeration: String,
}

// cmof:OpaqueExpression
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFOpaqueExpression {}

// cmof:Package
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFPackage {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_uri")]
    pub uri: String,
    #[serde(rename = "packageImport")]
    pub package_import: Option<Vec<CMOFPackageImport>>,
    #[serde(rename = "ownedMember")]
    pub owned_member: Option<Vec<EnumOwnedMember>>,
}

// cmof:PackageImport
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFPackageImport {
    #[serde(rename = "_xmi:type")]
    pub xmi_type: String,
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_importingNamespace")]
    pub importing_namespace: String,
    #[serde(rename = "importedPackage")]
    pub imported_package: ImportedPackage,
}

// cmof:PrimitiveType
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFPrimitiveType {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
}

// cmof:Property
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFProperty {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_visibility")]
    pub visibility: Option<String>,
    #[serde(rename = "_type")]
    pub r#type: Option<String>,
    #[serde(rename = "type")]
    pub complex_type: Option<ComplexType>,
    #[serde(rename = "_datatype")]
    pub datatype: Option<String>,
    #[serde(rename = "_lower")]
    pub lower: Option<String>,
    #[serde(rename = "_upper")]
    pub upper: Option<String>,
    #[serde(rename = "_default")]
    pub default: Option<String>,
    #[serde(rename = "_isReadOnly")]
    pub is_read_only: Option<String>,
    #[serde(rename = "_isComposite")]
    pub is_composite: Option<String>,
    #[serde(rename = "_isUnique")]
    pub is_unique: Option<String>,
    #[serde(rename = "_isOrdered")]
    pub is_ordered: Option<String>,
    #[serde(rename = "_isAbstract")]
    pub is_abstract: Option<String>,
    #[serde(rename = "_isDerived")]
    pub is_derived: Option<String>,
    #[serde(rename = "_subsettedProperty")]
    pub subsetted_property: Option<String>,
    #[serde(rename = "_owningAssociation")]
    pub owning_association: Option<String>,
    #[serde(rename = "_isDerivedUnion")]
    pub is_derived_union: Option<String>,
    #[serde(rename = "_association")]
    pub association: Option<String>,
    #[serde(rename = "redefinedProperty")]
    pub redefined_property_link: Option<EnumRedefinedProperty>,
    #[serde(rename = "subsettedProperty")]
    pub subsetted_property_link: Option<EnumSubsettedProperty>,
}

// cmof:Tag
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFTag {
    #[serde(rename = "_xmi:id")]
    pub xmi_id: String,
    #[serde(rename = "_name")]
    pub name: String,
    #[serde(rename = "_value")]
    pub value: String,
    #[serde(rename = "_element")]
    pub element: String,
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FilePackage {
    #[serde(rename = "cmof:Package")]
    pub package: CMOFPackage,
    #[serde(rename = "cmof:Tag")]
    tags: Vec<CMOFTag>,
    #[serde(rename = "_xmi:version")]
    xmi_versions: String,
    #[serde(rename = "_xmlns:xmi")]
    xmi_uri: String,
    #[serde(rename = "_xmlns:cmof")]
    cmof_uri: String,
    #[serde(rename = "_xmlns")]
    ns: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedAttribute {
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedEnd {
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedRule {
    #[serde(rename = "cmof:Constraint")]
    Constraint(CMOFConstraint),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedLiteral {
    #[serde(rename = "cmof:EnumerationLiteral")]
    EnumerationLiteral(CMOFEnumerationLiteral),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumRedefinedProperty {
    #[serde(rename = "cmof:Property")]
    Property(RedefinedProperty),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct RedefinedProperty {
    #[serde(rename = "_href")]
    href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumSubsettedProperty {
    #[serde(rename = "cmof:Property")]
    Property(SubsettedProperty),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct SubsettedProperty {
    #[serde(rename = "_href")]
    href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumSuperClass {
    #[serde(rename = "cmof:Class")]
    Class(SuperClass),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct SuperClass {
    #[serde(rename = "_href")]
    href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportedPackage {
    #[serde(rename = "_xmi:type")]
    pub r#type: String,
    #[serde(rename = "_href")]
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Specification {
    #[serde(rename = "_xmi:type")]
    xmi_type: String,
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "language")]
    language: String,
    #[serde(rename = "body")]
    body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct ComplexType {
    #[serde(rename = "_xmi:type")]
    xmi_type: String,
    #[serde(rename = "_href")]
    href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct OwnedLiteral {}

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
