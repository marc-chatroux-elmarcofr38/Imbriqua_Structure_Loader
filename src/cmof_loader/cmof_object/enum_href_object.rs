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
        _dict_setting: &mut BTreeMap<String, String>,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumImportedPackage::ImportedPackage(c) => set_href(&c.href, dict_object),
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
        _dict_setting: &mut BTreeMap<String, String>,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumRedefinedProperty::Property(c) => set_href(&c.href, dict_object),
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
        _dict_setting: &mut BTreeMap<String, String>,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumSubsettedProperty::Property(c) => set_href(&c.href, dict_object),
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
        _dict_setting: &mut BTreeMap<String, String>,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumSuperClass::HRefClass(c) => set_href(&c.href, dict_object),
        }
    }
}

impl EnumSuperClass {}

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
        _dict_setting: &mut BTreeMap<String, String>,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        match self {
            EnumType::HRefClass(c) => set_href(&c.href, dict_object),
            EnumType::HRefDataType(c) => set_href(&c.href, dict_object),
            EnumType::HRefPrimitiveType(c) => set_href(&c.href, dict_object),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Push content to RefCell in XMIIDReerence
pub fn set_href(
    reference: &XMIIdReference,
    dict_object: &mut BTreeMap<String, EnumCMOF>,
) -> Result<(), anyhow::Error> {
    // Criteria
    if reference.object.borrow().is_some() {
        panic!("'{:#?}' is already loaded", reference)
    };

    // Catch
    let k = reference.label()?;
    let r = dict_object.get(&k);
    if r.is_none() {
        return Err(anyhow::format_err!(
            "Matching error in post_deserialize : \"{}\" not find in dict_object",
            k
        ));
    } else {
        let v = r.unwrap();
        match v {
            EnumCMOF::CMOFAssociation(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFAssociation(c.clone())));
            }
            EnumCMOF::CMOFClass(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFClass(c.clone())));
            }
            EnumCMOF::CMOFConstraint(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFConstraint(c.clone())));
            }
            EnumCMOF::CMOFDataType(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFDataType(c.clone())));
            }
            EnumCMOF::CMOFEnumeration(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFEnumeration(c.clone())));
            }
            EnumCMOF::CMOFEnumerationLiteral(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFEnumerationLiteral(c.clone())));
            }
            EnumCMOF::CMOFOpaqueExpression(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFOpaqueExpression(c.clone())));
            }
            EnumCMOF::CMOFPackage(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFPackage(c.clone())));
            }
            EnumCMOF::CMOFPackageImport(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFPackageImport(c.clone())));
            }
            EnumCMOF::CMOFPrimitiveType(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFPrimitiveType(c.clone())));
            }
            EnumCMOF::CMOFProperty(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFProperty(c.clone())));
            }
            EnumCMOF::CMOFTag(c) => {
                let a = &reference.object;
                a.replace(Some(EnumCMOF::CMOFTag(c.clone())));
            }
        }
    }
    // Return
    Ok(())
}
