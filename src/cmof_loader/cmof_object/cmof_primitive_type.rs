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

#[derive(Clone, Debug, Deserialize, XMIIdentification)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF PrimitiveType Object
pub struct CMOFPrimitiveType {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// Casing formating of "name" as technical_name
    #[serde(skip)]
    pub parent: XMIIdReference<EnumWeakCMOF>,
    /// name attribute
    #[serde(rename = "_name")]
    name: String,
    /// Casing formating of "name" as technical_name
    #[serde(skip)]
    pub technical_name: String,
    /// Casing formating of "name" as table_name
    #[serde(skip)]
    pub table_name: String,
    /// Casing formating of "name" as model_case
    #[serde(skip)]
    pub model_name: String,
    /// Casing formating of "name" as full_name
    #[serde(skip)]
    pub full_name: String,
}

// ####################################################################################################
//
// ####################################################################################################

impl PartialEq for CMOFPrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.xmi_id == other.xmi_id
    }
}

impl Eq for CMOFPrimitiveType {}

impl PartialOrd for CMOFPrimitiveType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CMOFPrimitiveType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.xmi_id.cmp(&other.xmi_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFPrimitiveType {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        _dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Get needed values
        let package_name = dict_setting
            .get("package_name")
            .ok_or(anyhow::format_err!(
                "Dictionnary error in make_post_deserialize"
            ))?
            .clone();
        let parent_name = self.xmi_id.get_object_id();
        let package_name_snake_case = package_name.to_case(Case::Snake);
        let class_upper_case = self.name.to_case(Case::UpperCamel);
        let class_snake_case = self.name.to_case(Case::Snake);
        // Set local values
        self.xmi_id.set_package_id_if_empty(&package_name);
        self.technical_name = format!("{}.cmof#{}", package_name, self.name);
        self.table_name = format!("{}_{}", package_name_snake_case, class_snake_case);
        self.model_name = format!("{}", class_upper_case);
        self.full_name = format!("{}_primitive_{}", package_name_snake_case, class_snake_case);
        //Return
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Self
        set_xmi_id_object(&self.parent, dict_object)?;
        //Return
        Ok(())
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
