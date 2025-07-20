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

use std::collections::BTreeMap;

// Dependencies section
use serde::Deserialize;

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
/// Reference to another XMI object
pub struct XMIIdReference {
    object_id: String,
    package_id: String,
    is_set: bool,
}

impl XMIIdReference {
    /// Create when only object ID is available (need to use set_package after)
    pub fn new_local(object_id: String) -> Self {
        XMIIdReference {
            object_id: object_id,
            package_id: String::new(),
            is_set: false,
        }
    }

    /// Create when object ID and package ID are available (DON'T need to use set_package after)
    pub fn new_global(object_id: String, package_id: String) -> Self {
        XMIIdReference {
            object_id: object_id,
            package_id: package_id,
            is_set: true,
        }
    }

    /// Define package ID after a new_local
    pub fn set_package(&mut self, package_id: &String) {
        if !self.is_set {
            self.package_id = package_id.clone();
            self.is_set = true;
        }
    }

    /// Get object ID
    pub fn get_object_id(&self) -> &String {
        &self.object_id
    }

    /// Get package ID
    pub fn get_package_id(&self) -> &String {
        &self.package_id
    }

    /// Return combinaison of package ID and object ID
    pub fn label(&self) -> String {
        let mut r = self.package_id.clone();
        r.push_str("-");
        r.push_str(self.object_id.as_str());
        r
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Tools for CMOF Object
pub trait SetCMOFTools {
    /// Allow to define the post-treatment method : post_deserialize
    /// Make change after deserialization, on call
    /// Use "dict" for share content between parent object to child object
    fn make_post_deserialize(
        &mut self,
        dict: &mut BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error>;
}

/// Tool for CMOF Object
pub trait GetXMIId {
    /// Allow to get the xmi id field
    fn get_xmi_id_field(&self) -> String;
}
