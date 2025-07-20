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
pub use std::rc::Rc;

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Package Object
pub struct CMOFPackage {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdReference,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// uri attribute
    #[serde(rename = "_uri")]
    pub uri: String,
    /// Optional packageImport object array
    #[serde(rename = "packageImport")]
    #[serde(deserialize_with = "deser_btreemap_with_rc_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub package_import: BTreeMap<String, Rc<EnumPackageImport>>,
    /// Optional ownedMember object array
    #[serde(rename = "ownedMember")]
    #[serde(deserialize_with = "deser_btreemap_with_rc_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub owned_member: BTreeMap<String, Rc<EnumOwnedMember>>,
    /// Casing formating of "name" as technical_name
    #[serde(skip)]
    pub lowercase_name: String,
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFPackage {
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
        // Get needed values
        let package_name = self.name.clone();
        let package_name_snake_case = package_name.to_case(Case::Snake);
        dict_setting.insert(String::from("package_name"), package_name.clone());
        // Set local values
        self.xmi_id.set_package(&package_name);
        self.lowercase_name = String::from(package_name_snake_case);
        // Call on child
        for (_, p) in &mut self.package_import {
            let p_unwrap = Rc::get_mut(p).ok_or(anyhow::format_err!("\"Weak\" unwrap error"))?;
            p_unwrap.make_post_deserialize(dict_setting, dict_object)?;
        }
        for (_, p) in &mut self.owned_member {
            let p_unwrap = Rc::get_mut(p).ok_or(anyhow::format_err!("\"Weak\" unwrap error"))?;
            p_unwrap.make_post_deserialize(dict_setting, dict_object)?;
        }
        //Return
        Ok(())
    }
}

impl GetXMIId for CMOFPackage {
    fn get_xmi_id_field(&self) -> String {
        self.xmi_id.label()
    }
}
