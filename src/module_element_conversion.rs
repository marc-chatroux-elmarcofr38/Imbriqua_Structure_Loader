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
pub use quick_xml::de::from_str;
// use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
pub use serde_json;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

// cmof:Association
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFAssociation {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_visibility")]
    visibility: String,
    #[serde(rename = "_memberEnd")]
    member_end: Option<String>,
    #[serde(rename = "ownedEnd")]
    owned_end: Option<EnumOwnedEnd>,
}

// cmof:Class
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFClass {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_isAbstract")]
    is_abstract: Option<String>,
    #[serde(rename = "_superClass")]
    super_class: Option<String>,
    // #[serde(rename = "memberEnd")]
    // member_end: Option<String>,
    #[serde(rename = "ownedAttribute")]
    owned_attribute: Option<Vec<EnumOwnedAttribute>>,
    // #[serde(rename = "ownedEnd")]
    owned_end: Option<Vec<EnumOwnedEnd>>,
    #[serde(rename = "ownedRule")]
    owned_rule: Option<EnumOwnedRule>,
    // #[serde(rename = "ownedLiteral")]
    // owned_literal: Option<Vec<EnumOwnedLiteral>>,
}

// cmof:Constraint
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFConstraint {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_constrainedElement")]
    constrained_element: String,
    #[serde(rename = "_namespace")]
    namespace: String,
    #[serde(rename = "specification")]
    specification: Specification,
}

// cmof:DataType
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFDataType {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "ownedAttribute")]
    owned_attribute: Option<Vec<EnumOwnedAttribute>>,
    #[serde(rename = "ownedRule")]
    owned_rule: Option<EnumOwnedRule>,
}

// cmof:Enumeration
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFEnumeration {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "ownedLiteral")]
    owned_attribute: Option<Vec<EnumOwnedLiteral>>,
}

// cmof:EnumerationLiteral
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFEnumerationLiteral {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_classifier")]
    classifier: String,
    #[serde(rename = "_enumeration")]
    enumeration: String,
}

// cmof:OpaqueExpression
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFOpaqueExpression {}

// cmof:Package
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CMOFPackage {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_uri")]
    uri: String,
    #[serde(rename = "packageImport")]
    package_import: Option<Vec<CMOFPackageImport>>,
    #[serde(rename = "ownedMember")]
    owned_member: Option<Vec<EnumOwnedMember>>,
}

// cmof:PackageImport
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFPackageImport {
    #[serde(rename = "_xmi:type")]
    xmi_type: String,
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_importingNamespace")]
    importing_namespace: String,
    #[serde(rename = "importedPackage")]
    imported_package: ImportedPackage,
}

// cmof:PrimitiveType
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFPrimitiveType {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
}

// cmof:Property
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFProperty {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_visibility")]
    visibility: Option<String>,
    #[serde(rename = "_type")]
    r#type: Option<String>,
    #[serde(rename = "type")]
    complex_type: Option<ComplexType>,
    #[serde(rename = "_datatype")]
    datatype: Option<String>,
    #[serde(rename = "_lower")]
    lower: Option<String>,
    #[serde(rename = "_upper")]
    upper: Option<String>,
    #[serde(rename = "_default")]
    default: Option<String>,
    #[serde(rename = "_isReadOnly")]
    is_read_only: Option<String>,
    #[serde(rename = "_isComposite")]
    is_composite: Option<String>,
    #[serde(rename = "_isUnique")]
    is_unique: Option<String>,
    #[serde(rename = "_isOrdered")]
    is_ordered: Option<String>,
    #[serde(rename = "_isAbstract")]
    is_abstract: Option<String>,
    #[serde(rename = "_isDerived")]
    is_derived: Option<String>,
    #[serde(rename = "_subsettedProperty")]
    subsetted_property: Option<String>,
    #[serde(rename = "_owningAssociation")]
    owning_association: Option<String>,

    #[serde(rename = "_isDerivedUnion")]
    is_derived_union: Option<String>,
    #[serde(rename = "_association")]
    association: Option<String>,
}

// cmof:Tag
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct CMOFTag {
    #[serde(rename = "_xmi:id")]
    xmi_id: String,
    #[serde(rename = "_name")]
    name: String,
    #[serde(rename = "_value")]
    value: String,
    #[serde(rename = "_element")]
    element: String,
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FilePackage {
    #[serde(rename = "cmof:Package")]
    pub packages: CMOFPackage,
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedAttribute {
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedEnd {
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedRule {
    #[serde(rename = "cmof:Constraint")]
    Constraint(CMOFConstraint),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
enum EnumOwnedLiteral {
    #[serde(rename = "cmof:EnumerationLiteral")]
    EnumerationLiteral(CMOFEnumerationLiteral),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct ImportedPackage {
    #[serde(rename = "_xmi:type")]
    r#type: String,
    #[serde(rename = "_href")]
    href: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct ComplexType {
    #[serde(rename = "_xmi:type")]
    xmi_type: String,
    #[serde(rename = "_href")]
    href: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
