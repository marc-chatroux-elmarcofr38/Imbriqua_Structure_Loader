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
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Enumeration Object
pub struct CMOFEnumeration {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdReference,
    /// name attribute
    #[serde(rename = "_name")]
    pub name: String,
    /// Optional ownedLiteral object arry
    #[serde(rename = "ownedLiteral")]
    #[serde(deserialize_with = "deser_btreemap_with_rc_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub owned_attribute: BTreeMap<String, Rc<EnumOwnedLiteral>>,
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

impl SetCMOFTools for CMOFEnumeration {
    fn make_post_deserialize(
        &mut self,
        dict: &mut BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error> {
        // Get needed values
        let package_name = dict.get("package_name").ok_or(anyhow::format_err!(
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
            let p_unwrap = Rc::get_mut(p).ok_or(anyhow::format_err!("\"Weak\" unwrap error"))?;
            p_unwrap.make_post_deserialize(dict)?;
        }
        //Return
        Ok(())
    }
}

impl GetXMIId for CMOFEnumeration {
    fn get_xmi_id_field(&self) -> String {
        self.xmi_id.label()
    }
}
