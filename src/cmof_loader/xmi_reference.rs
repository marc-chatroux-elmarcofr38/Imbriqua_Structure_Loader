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

use anyhow::Error;
// Dependencies section
use serde::Deserialize;

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug, Deserialize, PartialEq)]
/// Reference to another XMI object
pub struct XMIIdReference {
    local_id: String,
    package_id: String,
    is_set: bool,
}

impl XMIIdReference {
    /// Create when only local ID is available (need to use set_package after)
    pub fn new_local(local_id: String) -> Self {
        XMIIdReference {
            local_id: local_id,
            package_id: String::new(),
            is_set: false,
        }
    }

    pub fn new_global(local_id: String, package_id: String) -> Self {
        XMIIdReference {
            local_id: local_id,
            package_id: package_id,
            is_set: true,
        }
    }

    pub fn set_package(&mut self, package_id: &String) {
        if !self.is_set {
            self.package_id = package_id.clone();
            self.is_set = true;
        }
    }

    pub fn get_local_id(&self) -> &String {
        &self.local_id
    }

    pub fn get_package_id(&self) -> &String {
        &self.package_id
    }

    pub fn label(&self) -> String {
        let mut r = self.package_id.clone();
        r.push_str("-");
        r.push_str(self.local_id.as_str());
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
    fn make_post_deserialize(
        &mut self,
        dict: &mut BTreeMap<String, String>,
    ) -> Result<(), anyhow::Error>;
}
