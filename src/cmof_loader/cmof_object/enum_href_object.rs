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
            EnumImportedPackage::ImportedPackage(c) => c.set_href(dict_object),
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
            EnumRedefinedProperty::Property(c) => c.set_href(dict_object),
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
            EnumSubsettedProperty::Property(c) => c.set_href(dict_object),
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
            EnumSuperClass::HRefClass(c) => c.set_href(dict_object),
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
            EnumType::HRefClass(c) => c.set_href(dict_object),
            EnumType::HRefDataType(c) => c.set_href(dict_object),
            EnumType::HRefPrimitiveType(c) => c.set_href(dict_object),
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn test_01_creation() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            panic!();

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }
}
