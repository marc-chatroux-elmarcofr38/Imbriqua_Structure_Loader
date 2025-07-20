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
use crate::loader_deserialise_helper::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Constraint Object
pub struct CMOFConstraint {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdReference,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// constrainedElement attribute
    #[serde(rename = "_constrainedElement")]
    pub constrained_element: String,
    /// namespace attribute
    #[serde(rename = "_namespace")]
    pub namespace: String,
    /// specification object
    #[serde(rename = "specification")]
    pub specification: EnumSpecification,
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFConstraint {
    fn make_post_deserialize(
        &mut self,
        dict: &mut BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        // Get needed values
        let package_name = dict.get("package_name").ok_or(anyhow::format_err!(
            "Dictionnary error in make_post_deserialize"
        ))?;
        // Set local values
        self.xmi_id.set_package(&package_name);
        // Call on child
        self.specification.make_post_deserialize(dict)?;
        //Return
        Ok(())
    }
}
