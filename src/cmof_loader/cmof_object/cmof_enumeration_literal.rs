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
use serde::Deserialize;
use std::collections::BTreeMap;

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF EnumerationLiteral Object
pub struct CMOFEnumerationLiteral {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// name attribute
    #[serde(rename = "_name")]
    name: String,
    /// classifier attribute
    #[serde(rename = "_classifier")]
    _classifier: String,
    /// enumeration attribute
    #[serde(rename = "_enumeration")]
    _enumeration: String,
    /// Casing formating of "name" as table_name
    #[serde(skip)]
    pub litteral_name: String,
    /// Casing formating of "name" as table_name
    #[serde(skip)]
    pub litteral_designation: String,
}

// ####################################################################################################
//
// ####################################################################################################

impl PartialEq for CMOFEnumerationLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.xmi_id == other.xmi_id
    }
}

impl Eq for CMOFEnumerationLiteral {}

impl PartialOrd for CMOFEnumerationLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CMOFEnumerationLiteral {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.xmi_id.cmp(&other.xmi_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFEnumerationLiteral {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Get needed values
        let package_name = dict_setting.get("package_name").ok_or(anyhow::format_err!(
            "Dictionnary error in make_post_deserialize"
        ))?;
        // Set local values
        self.xmi_id.set_package(&package_name);
        self.litteral_designation = self.name.clone();
        self.litteral_name = self.name.to_case(Case::UpperCamel);
        //Return
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        //Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl GetXMIId for CMOFEnumerationLiteral {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        self.xmi_id.label()
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        Ok(self.xmi_id.get_object_id())
    }
}
