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

#[derive(Clone, Debug, Deserialize, XMIIdentification)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Constraint Object
pub struct CMOFConstraint {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// Casing formating of "name" as technical_name
    #[serde(skip)]
    pub parent: XMIIdReference<EnumWeakCMOF>,
    /// name attribute
    #[serde(rename = "_name")]
    _name: String,
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

impl PartialEq for CMOFConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.xmi_id == other.xmi_id
    }
}

impl Eq for CMOFConstraint {}

impl PartialOrd for CMOFConstraint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CMOFConstraint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.xmi_id.cmp(&other.xmi_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFConstraint {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Get needed values
        let package_name = dict_setting
            .get("package_name")
            .ok_or(anyhow::format_err!(
                "Dictionnary error in make_post_deserialize"
            ))?
            .clone();
        let parent_name = self.xmi_id.get_object_id();
        // Set local values
        self.xmi_id.set_package_id_if_empty(&package_name);
        // Call on child
        match &mut self.specification {
            EnumSpecification::OpaqueExpression(c) => {
                let m = Rc::get_mut(c).unwrap();
                m.parent.set_package_id_if_empty(&package_name);
                m.parent.set_object_id(&parent_name);
                m.collect_object(dict_setting, dict_object)?;
                dict_object.insert(
                    c.get_xmi_id_field()?,
                    EnumCMOF::CMOFOpaqueExpression(c.clone()),
                );
            }
        }
        //Return
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Call on child
        match &self.specification {
            EnumSpecification::OpaqueExpression(c) => {
                c.make_post_deserialize(dict_object)?;
            }
        }
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
