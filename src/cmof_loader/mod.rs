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
#![doc = include_str!("../../doc/loader_cmof_structure.md")]

// Mod section
mod cmof_object;
pub use cmof_object::*;
mod href;
pub use href::*;
mod xmi_reference;
pub use xmi_reference::*;

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_post_treament_deserialize::*;

// Dependencies section
pub use serde::Deserialize;
pub use std::collections::BTreeMap;
pub use std::rc::{Rc, Weak};

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
/// RUST Struct for representing package file
pub struct FilePackage {
    /// cmof:Package object
    #[serde(deserialize_with = "deser_post_treatement_cmof_package")]
    #[serde(rename = "cmof:Package")]
    pub package: CMOFPackage,
    /// cmof:Tag object list
    #[serde(rename = "cmof:Tag")]
    pub tags: Vec<CMOFTag>,
    /// xmi version
    #[serde(rename = "_xmi:version")]
    pub xmi_versions: String,
    /// XLM namespace XMI
    #[serde(rename = "_xmlns:xmi")]
    pub xmi_uri: String,
    /// XML namespace CMOF
    #[serde(rename = "_xmlns:cmof")]
    pub cmof_uri: String,
    /// XML namespace
    #[serde(rename = "_xmlns")]
    pub ns: String,
}
