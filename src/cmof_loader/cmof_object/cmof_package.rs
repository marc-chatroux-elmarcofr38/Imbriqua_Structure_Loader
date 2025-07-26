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

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// RUST Struct for deserialize CMOF Package Object
pub struct CMOFPackage {
    /// xmi:id attribute
    #[serde(deserialize_with = "deser_local_xmi_id")]
    #[serde(rename = "_xmi:id")]
    pub xmi_id: XMIIdLocalReference,
    /// name attribute
    #[serde(rename = "_name")]
    name: String,
    /// uri attribute
    #[serde(rename = "_uri")]
    pub uri: String,
    /// Optional packageImport object array
    #[serde(rename = "packageImport")]
    #[serde(deserialize_with = "deser_btreemap_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub package_import: BTreeMap<String, EnumPackageImport>,
    /// Optional ownedMember object array
    #[serde(rename = "ownedMember")]
    #[serde(deserialize_with = "deser_btreemap_using_name_as_key")]
    #[serde(default = "default_empty_btreemap")]
    pub owned_member: BTreeMap<String, EnumOwnedMember>,
    /// Casing formating of "name" as technical_name
    #[serde(skip)]
    pub lowercase_name: String,
}

// ####################################################################################################
//
// ####################################################################################################

impl PartialEq for CMOFPackage {
    fn eq(&self, other: &Self) -> bool {
        self.xmi_id == other.xmi_id
    }
}

impl Eq for CMOFPackage {}

impl PartialOrd for CMOFPackage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CMOFPackage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.xmi_id.cmp(&other.xmi_id)
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl SetCMOFTools for CMOFPackage {
    fn collect_object(
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
            match p {
                EnumPackageImport::PackageImport(ref mut c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(
                        c.get_xmi_id_field()?,
                        EnumCMOF::CMOFPackageImport(c.clone()),
                    );
                }
            }
        }
        for (_, p) in &mut self.owned_member {
            match p {
                EnumOwnedMember::Association(ref mut c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFAssociation(c.clone()));
                }
                EnumOwnedMember::Class(ref mut c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFClass(c.clone()));
                }
                EnumOwnedMember::DataType(ref mut c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFDataType(c.clone()));
                }
                EnumOwnedMember::Enumeration(ref mut c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(c.get_xmi_id_field()?, EnumCMOF::CMOFEnumeration(c.clone()));
                }
                EnumOwnedMember::PrimitiveType(ref mut c) => {
                    let m = Rc::get_mut(c).unwrap();
                    m.collect_object(dict_setting, dict_object)?;
                    dict_object.insert(
                        c.get_xmi_id_field()?,
                        EnumCMOF::CMOFPrimitiveType(c.clone()),
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
        for (_, p) in &self.package_import {
            p.make_post_deserialize(dict_object)?;
        }
        for (_, p) in &self.owned_member {
            p.make_post_deserialize(dict_object)?;
        }
        //Return
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

impl GetXMIId for CMOFPackage {
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error> {
        self.xmi_id.label()
    }
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error> {
        Ok(self.xmi_id.get_object_id())
    }
}
