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

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF OpaqueExpression Object
pub struct CMOFOpaqueExpression {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// body attribute
    #[serde(rename = "body")]
    pub body: String,
    /// language attribute
    #[serde(rename = "language")]
    pub language: String,
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFOpaqueExpression {
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

impl GetXMIId for CMOFOpaqueExpression {
    fn get_xmi_id_field(&self) -> String {
        self.xmi_id.label()
    }
    fn get_xmi_id_object(&self) -> String {
        self.xmi_id.get_object_id()
    }
}
