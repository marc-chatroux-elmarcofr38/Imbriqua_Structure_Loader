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

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Enumeration Object
pub struct CMOFEnumeration {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// name attribute
    #[serde(rename = "_name")]
    name: String,
    /// Optional ownedLiteral object arry
    #[serde(rename = "ownedLiteral")]
    #[serde(deserialize_with = "deser_btreemap_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub owned_attribute: BTreeMap<String, EnumOwnedLiteral>,
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

impl PartialEq for CMOFEnumeration {
    fn eq(&self, other: &Self) -> bool {
        self.xmi_id == other.xmi_id
    }
}

impl Eq for CMOFEnumeration {}

impl PartialOrd for CMOFEnumeration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CMOFEnumeration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.xmi_id.cmp(&other.xmi_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFEnumeration {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        // Get needed values
        let package_name = dict_setting.get("package_name").ok_or(anyhow::format_err!(
            "Dictionnary error in make_post_deserialize"
        ))?;
        let package_name_snake_case = package_name.to_case(Case::Snake);
        let class_upper_case = self.name.to_case(Case::UpperCamel);
        let class_snake_case = self.name.to_case(Case::Snake);
        // Set local values
        self.xmi_id.set_package(&package_name);
        self.technical_name = format!("{}.cmof#{}", package_name, self.name);
        self.table_name = format!("{}_{}", package_name_snake_case, class_snake_case);
        self.model_name = format!("{}", class_upper_case);
        self.full_name = format!(
            "{}_enumeration_{}",
            package_name_snake_case, class_snake_case
        );
        // Call on child
        for (_, p) in &mut self.owned_attribute {
            match p {
                EnumOwnedLiteral::EnumerationLiteral(ref mut c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(
                        c.get_xmi_id_field()?,
                        EnumCMOF::CMOFEnumerationLiteral(c.clone()),
                    );
                }
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
        for (_, p) in &self.owned_attribute {
            p.make_post_deserialize(dict_object)?;
        }
        //Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl GetXMIId for CMOFEnumeration {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        self.xmi_id.label()
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        Ok(self.xmi_id.get_object_id())
    }
}
