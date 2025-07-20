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

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// All object with xmi_id in CMOF
pub enum EnumCMOF {
    /// CMOFAssociation
    CMOFAssociation(Weak<CMOFAssociation>),
    /// CMOFClass
    CMOFClass(Weak<CMOFClass>),
    /// CMOFConstraint
    CMOFConstraint(Weak<CMOFConstraint>),
    /// CMOFDataType
    CMOFDataType(Weak<CMOFDataType>),
    /// CMOFEnumeration
    CMOFEnumeration(Weak<CMOFEnumeration>),
    /// CMOFEnumerationLiteral
    CMOFEnumerationLiteral(Weak<CMOFEnumerationLiteral>),
    /// CMOFOpaqueExpression
    CMOFOpaqueExpression(Weak<CMOFOpaqueExpression>),
    /// CMOFPackage
    CMOFPackage(Weak<CMOFPackage>),
    /// CMOFPackageImport
    CMOFPackageImport(Weak<CMOFPackageImport>),
    /// CMOFPrimitiveType
    CMOFPrimitiveType(Weak<CMOFPrimitiveType>),
    /// CMOFProperty
    CMOFProperty(Weak<CMOFProperty>),
    /// CMOFTag
    CMOFTag(Weak<CMOFTag>),
}

impl EnumCMOF {
    /// Get "label" of the xmi_id
    pub fn label(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumCMOF::CMOFAssociation(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFClass(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFConstraint(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFDataType(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFEnumeration(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFEnumerationLiteral(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFOpaqueExpression(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFPackage(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFPackageImport(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFPrimitiveType(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFProperty(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
            EnumCMOF::CMOFTag(c) => {
                let r = c.upgrade();
                if r.is_some() {
                    Ok(r.unwrap().xmi_id.label())
                } else {
                    Err(anyhow::format_err!("Getting label of a unloaded Weak"))
                }
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Complex Class link object
pub enum EnumImportedPackage {
    /// ImportedPackage Tag with cmof:Package type
    #[serde(rename = "cmof:Package")]
    ImportedPackage(HRefImportedPackage),
}

impl SetCMOFTools for EnumImportedPackage {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumImportedPackage::ImportedPackage(c) => {
                c.make_post_deserialize(dict_setting, dict_object)
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedAttribute Tag
pub enum EnumOwnedAttribute {
    /// OwnedAttribute with cmof:Property type
    #[serde(rename = "cmof:Property")]
    Property(CMOFProperty),
}

impl SetCMOFTools for EnumOwnedAttribute {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedAttribute::Property(c) => c.make_post_deserialize(dict_setting, dict_object),
        }
    }
}

impl GetXMIId for EnumOwnedAttribute {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedAttribute::Property(c) => c.get_xmi_id_field(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedEnd Tag
pub enum EnumOwnedEnd {
    /// OwnedEnd with cmof:Property type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Property")]
    Property(Rc<CMOFProperty>),
}

impl SetCMOFTools for EnumOwnedEnd {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedEnd::Property(c) => {
                let m = Rc::get_mut(c);
                if m.is_some() {
                    m.unwrap().make_post_deserialize(dict_setting, dict_object);
                    Ok(())
                } else {
                    Err(anyhow::format_err!("Error getting Mut of a Rc"))
                }
            }
        }
    }
}

impl GetXMIId for EnumOwnedEnd {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedEnd::Property(c) => c.get_xmi_id_field(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedLiteral Tag
pub enum EnumOwnedLiteral {
    /// OwnedLiteral with cmof:EnumerationLiteral type
    #[serde(rename = "cmof:EnumerationLiteral")]
    EnumerationLiteral(CMOFEnumerationLiteral),
}

impl SetCMOFTools for EnumOwnedLiteral {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(c) => {
                c.make_post_deserialize(dict_setting, dict_object)
            }
        }
    }
}

impl GetXMIId for EnumOwnedLiteral {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(c) => c.get_xmi_id_field(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

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
    /// OwnedMember with cmof:DataType type
    #[serde(rename = "cmof:DataType")]
    DataType(CMOFDataType),
    /// OwnedMember with cmof:Enumeration type
    #[serde(rename = "cmof:Enumeration")]
    Enumeration(CMOFEnumeration),
    /// OwnedMember with cmof:PrimitiveType type
    #[serde(rename = "cmof:PrimitiveType")]
    PrimitiveType(CMOFPrimitiveType),
}

impl SetCMOFTools for EnumOwnedMember {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedMember::Association(c) => c.make_post_deserialize(dict_setting, dict_object),
            EnumOwnedMember::Class(c) => c.make_post_deserialize(dict_setting, dict_object),
            EnumOwnedMember::DataType(c) => c.make_post_deserialize(dict_setting, dict_object),
            EnumOwnedMember::Enumeration(c) => c.make_post_deserialize(dict_setting, dict_object),
            EnumOwnedMember::PrimitiveType(c) => c.make_post_deserialize(dict_setting, dict_object),
        }
    }
}

impl GetXMIId for EnumOwnedMember {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedMember::Association(c) => c.get_xmi_id_field(),
            EnumOwnedMember::Class(c) => c.get_xmi_id_field(),
            EnumOwnedMember::DataType(c) => c.get_xmi_id_field(),
            EnumOwnedMember::Enumeration(c) => c.get_xmi_id_field(),
            EnumOwnedMember::PrimitiveType(c) => c.get_xmi_id_field(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing OwnedRule Tag
pub enum EnumOwnedRule {
    /// OwnedRule with cmof:Constraint type
    #[serde(rename = "cmof:Constraint")]
    Constraint(CMOFConstraint),
}

impl SetCMOFTools for EnumOwnedRule {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedRule::Constraint(c) => c.make_post_deserialize(dict_setting, dict_object),
        }
    }
}

impl GetXMIId for EnumOwnedRule {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedRule::Constraint(c) => c.get_xmi_id_field(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing PackageImport Tag
pub enum EnumPackageImport {
    /// PackageImport Tag with cmof:PackageImport type
    #[serde(rename = "cmof:PackageImport")]
    PackageImport(CMOFPackageImport),
}

impl SetCMOFTools for EnumPackageImport {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumPackageImport::PackageImport(c) => {
                c.make_post_deserialize(dict_setting, dict_object)
            }
        }
    }
}

impl GetXMIId for EnumPackageImport {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumPackageImport::PackageImport(c) => c.get_xmi_id_field(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing RedefinedProperty Tag
pub enum EnumRedefinedProperty {
    /// RedefinedProperty with cmof:Property type
    #[serde(rename = "cmof:Property")]
    Property(HRefRedefinedProperty),
}

impl SetCMOFTools for EnumRedefinedProperty {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumRedefinedProperty::Property(c) => {
                c.make_post_deserialize(dict_setting, dict_object)
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Specification Tag
pub enum EnumSpecification {
    /// Specification Tag with cmof:OpaqueExpression type
    #[serde(rename = "cmof:OpaqueExpression")]
    OpaqueExpression(CMOFOpaqueExpression),
}

impl SetCMOFTools for EnumSpecification {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumSpecification::OpaqueExpression(c) => {
                c.make_post_deserialize(dict_setting, dict_object)
            }
        }
    }
}

impl GetXMIId for EnumSpecification {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumSpecification::OpaqueExpression(c) => c.get_xmi_id_field(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing SubsettedProperty Tag
pub enum EnumSubsettedProperty {
    /// SubsettedProperty with cmof:Property type
    #[serde(rename = "cmof:Property")]
    Property(HRefSubsettedProperty),
}

impl SetCMOFTools for EnumSubsettedProperty {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumSubsettedProperty::Property(c) => {
                c.make_post_deserialize(dict_setting, dict_object)
            }
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Complex Class link object
pub enum EnumSuperClass {
    /// SuperClass Tag with cmof:Class type
    #[serde(rename = "cmof:Class")]
    HRefClass(HRefClass),
}

impl SetCMOFTools for EnumSuperClass {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumSuperClass::HRefClass(c) => c.make_post_deserialize(dict_setting, dict_object),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "_xmi:type")]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing Complex Class link object
pub enum EnumType {
    /// ImportedPackage Tag with cmof:Class type
    #[serde(rename = "cmof:Class")]
    HRefClass(HRefClass),
    /// ImportedPackage Tag with cmof:PrimitiveType type
    #[serde(rename = "cmof:PrimitiveType")]
    HRefPrimitiveType(HRefPrimitiveType),
    /// ImportedPackage Tag with cmof:DataType type
    #[serde(rename = "cmof:DataType")]
    HRefDataType(HRefDataType),
}

impl SetCMOFTools for EnumType {
    fn collect_object(
        &mut self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumType::HRefClass(c) => c.make_post_deserialize(dict_setting, dict_object),
            EnumType::HRefDataType(c) => c.make_post_deserialize(dict_setting, dict_object),
            EnumType::HRefPrimitiveType(c) => c.make_post_deserialize(dict_setting, dict_object),
        }
    }
}
