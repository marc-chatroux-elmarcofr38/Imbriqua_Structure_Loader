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

use anyhow::Ok;

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
    CMOFAssociation(Rc<CMOFAssociation>),
    /// CMOFClass
    CMOFClass(Rc<CMOFClass>),
    /// CMOFConstraint
    CMOFConstraint(Rc<CMOFConstraint>),
    /// CMOFDataType
    CMOFDataType(Rc<CMOFDataType>),
    /// CMOFEnumeration
    CMOFEnumeration(Rc<CMOFEnumeration>),
    /// CMOFEnumerationLiteral
    CMOFEnumerationLiteral(Rc<CMOFEnumerationLiteral>),
    /// CMOFOpaqueExpression
    CMOFOpaqueExpression(Rc<CMOFOpaqueExpression>),
    /// CMOFPackage
    CMOFPackage(Rc<CMOFPackage>),
    /// CMOFPackageImport
    CMOFPackageImport(Rc<CMOFPackageImport>),
    /// CMOFPrimitiveType
    CMOFPrimitiveType(Rc<CMOFPrimitiveType>),
    /// CMOFProperty
    CMOFProperty(Rc<CMOFProperty>),
    /// CMOFTag
    CMOFTag(Rc<CMOFTag>),
}

impl EnumCMOF {
    /// Get "label" of the xmi_id
    pub fn label(&self) -> Result<String, anyhow::Error> {
        match self {
            EnumCMOF::CMOFAssociation(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFClass(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFConstraint(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFDataType(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFEnumeration(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFEnumerationLiteral(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFOpaqueExpression(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFPackage(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFPackageImport(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFPrimitiveType(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFProperty(c) => Ok(c.xmi_id.label()),
            EnumCMOF::CMOFTag(c) => Ok(c.xmi_id.label()),
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
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Property")]
    Property(Rc<CMOFProperty>),
}

impl SetCMOFTools for EnumOwnedAttribute {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedAttribute::Property(c) => {
                let m = Rc::get_mut(c).unwrap();
                m.collect_object(dict_setting, dict_object)?;
                dict_object.insert(c.get_xmi_id_field(), EnumCMOF::CMOFProperty(c.clone()));
            }
        }
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedAttribute::Property(c) => catch_and_make_post_deserialize(c, dict_object),
        }
    }
}

impl GetXMIId for EnumOwnedAttribute {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedAttribute::Property(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> String {
        match self {
            EnumOwnedAttribute::Property(c) => c.get_xmi_id_object(),
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
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedEnd::Property(c) => {
                let m = Rc::get_mut(c).unwrap();
                m.collect_object(dict_setting, dict_object)?;
                dict_object.insert(c.get_xmi_id_field(), EnumCMOF::CMOFProperty(c.clone()));
            }
        }
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedEnd::Property(c) => catch_and_make_post_deserialize(c, dict_object),
        }
    }
}

impl GetXMIId for EnumOwnedEnd {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedEnd::Property(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> String {
        match self {
            EnumOwnedEnd::Property(c) => c.get_xmi_id_object(),
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
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:EnumerationLiteral")]
    EnumerationLiteral(Rc<CMOFEnumerationLiteral>),
}

impl SetCMOFTools for EnumOwnedLiteral {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(c) => {
                catch_and_collect_object(c, dict_setting, dict_object)
            }
        }
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(c) => {
                catch_and_make_post_deserialize(c, dict_object)
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
    fn get_xmi_id_object(&self) -> String {
        match self {
            EnumOwnedLiteral::EnumerationLiteral(c) => c.get_xmi_id_object(),
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
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Association")]
    Association(Rc<CMOFAssociation>),
    /// OwnedMember with cmof:Class type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Class")]
    Class(Rc<CMOFClass>),
    /// OwnedMember with cmof:DataType type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:DataType")]
    DataType(Rc<CMOFDataType>),
    /// OwnedMember with cmof:Enumeration type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Enumeration")]
    Enumeration(Rc<CMOFEnumeration>),
    /// OwnedMember with cmof:PrimitiveType type
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:PrimitiveType")]
    PrimitiveType(Rc<CMOFPrimitiveType>),
}

impl SetCMOFTools for EnumOwnedMember {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedMember::Association(c) => {
                catch_and_collect_object(c, dict_setting, dict_object)
            }
            EnumOwnedMember::Class(c) => catch_and_collect_object(c, dict_setting, dict_object),
            EnumOwnedMember::DataType(c) => catch_and_collect_object(c, dict_setting, dict_object),
            EnumOwnedMember::Enumeration(c) => {
                catch_and_collect_object(c, dict_setting, dict_object)
            }
            EnumOwnedMember::PrimitiveType(c) => {
                catch_and_collect_object(c, dict_setting, dict_object)
            }
        }
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedMember::Association(c) => catch_and_make_post_deserialize(c, dict_object),
            EnumOwnedMember::Class(c) => catch_and_make_post_deserialize(c, dict_object),
            EnumOwnedMember::DataType(c) => catch_and_make_post_deserialize(c, dict_object),
            EnumOwnedMember::Enumeration(c) => catch_and_make_post_deserialize(c, dict_object),
            EnumOwnedMember::PrimitiveType(c) => catch_and_make_post_deserialize(c, dict_object),
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
    fn get_xmi_id_object(&self) -> String {
        match self {
            EnumOwnedMember::Association(c) => c.get_xmi_id_object(),
            EnumOwnedMember::Class(c) => c.get_xmi_id_object(),
            EnumOwnedMember::DataType(c) => c.get_xmi_id_object(),
            EnumOwnedMember::Enumeration(c) => c.get_xmi_id_object(),
            EnumOwnedMember::PrimitiveType(c) => c.get_xmi_id_object(),
        }
    }
}

impl NamingStruct for EnumOwnedMember {
    fn get_technical_name(&self, _package: &LoadingPackage) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.technical_name.clone(),
            EnumOwnedMember::Class(content) => content.technical_name.clone(),
            EnumOwnedMember::DataType(content) => content.technical_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.technical_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.technical_name.clone(),
        }
    }
    fn get_table_name(&self, _package: &LoadingPackage) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.table_name.clone(),
            EnumOwnedMember::Class(content) => content.table_name.clone(),
            EnumOwnedMember::DataType(content) => content.table_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.table_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.table_name.clone(),
        }
    }
    fn get_model_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.model_name.clone(),
            EnumOwnedMember::Class(content) => content.model_name.clone(),
            EnumOwnedMember::DataType(content) => content.model_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.model_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.model_name.clone(),
        }
    }
    fn get_full_name(&self, _package: &LoadingPackage) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.full_name.clone(),
            EnumOwnedMember::Class(content) => content.full_name.clone(),
            EnumOwnedMember::DataType(content) => content.full_name.clone(),
            EnumOwnedMember::Enumeration(content) => content.full_name.clone(),
            EnumOwnedMember::PrimitiveType(content) => content.full_name.clone(),
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
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:Constraint")]
    Constraint(Rc<CMOFConstraint>),
}

impl SetCMOFTools for EnumOwnedRule {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedRule::Constraint(c) => {
                let m = Rc::get_mut(c).unwrap();
                m.collect_object(dict_setting, dict_object)?;
                dict_object.insert(c.get_xmi_id_field(), EnumCMOF::CMOFConstraint(c.clone()));
            }
        }
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumOwnedRule::Constraint(c) => catch_and_make_post_deserialize(c, dict_object),
        }
    }
}

impl GetXMIId for EnumOwnedRule {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumOwnedRule::Constraint(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> String {
        match self {
            EnumOwnedRule::Constraint(c) => c.get_xmi_id_object(),
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
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:PackageImport")]
    PackageImport(Rc<CMOFPackageImport>),
}

impl SetCMOFTools for EnumPackageImport {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumPackageImport::PackageImport(c) => {
                let m = Rc::get_mut(c).unwrap();
                m.collect_object(dict_setting, dict_object)?;
                dict_object.insert(c.get_xmi_id_field(), EnumCMOF::CMOFPackageImport(c.clone()));
            }
        }
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumPackageImport::PackageImport(c) => catch_and_make_post_deserialize(c, dict_object),
        }
    }
}

impl GetXMIId for EnumPackageImport {
    fn get_xmi_id_field(&self) -> String {
        match self {
            EnumPackageImport::PackageImport(c) => c.get_xmi_id_field(),
        }
    }
    fn get_xmi_id_object(&self) -> String {
        match self {
            EnumPackageImport::PackageImport(c) => c.get_xmi_id_object(),
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
    #[serde(deserialize_with = "deser_rc")]
    #[serde(rename = "cmof:OpaqueExpression")]
    OpaqueExpression(Rc<CMOFOpaqueExpression>),
}

impl SetCMOFTools for EnumSpecification {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumSpecification::OpaqueExpression(c) => {
                let m = Rc::get_mut(c).unwrap();
                m.collect_object(dict_setting, dict_object)?;
                dict_object.insert(
                    c.get_xmi_id_field(),
                    EnumCMOF::CMOFOpaqueExpression(c.clone()),
                );
            }
        }
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumSpecification::OpaqueExpression(c) => {
                catch_and_make_post_deserialize(c, dict_object)
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
    fn get_xmi_id_object(&self) -> String {
        match self {
            EnumSpecification::OpaqueExpression(c) => c.get_xmi_id_object(),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

fn catch_and_collect_object<T: SetCMOFTools>(
    object: &mut Rc<T>,
    dict_setting: &mut BTreeMap<String, String>,
    dict_object: &mut BTreeMap<String, EnumCMOF>,
) -> Result<(), anyhow::Error> {
    let m = Rc::get_mut(object);
    if m.is_some() {
        m.unwrap().collect_object(dict_setting, dict_object)?;
        Ok(())
    } else {
        Err(anyhow::format_err!(
            "Error getting Mut of a Rc (catch_and_collect_object)"
        ))
    }
}

// ####################################################################################################
//
// ####################################################################################################

fn catch_and_make_post_deserialize<T: SetCMOFTools>(
    object: &Rc<T>,
    dict_object: &mut BTreeMap<String, EnumCMOF>,
) -> Result<(), anyhow::Error> {
    object.make_post_deserialize(dict_object)?;
    Ok(())
}
