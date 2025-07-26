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
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone)]
/// Reference to another XMI object
pub struct XMIIdLocalReference {
    object_id: String,
    package_id: String,
    is_set: bool,
}

impl PartialEq for XMIIdLocalReference {
    fn eq(&self, other: &Self) -> bool {
        self.object_id == other.object_id
            && self.package_id == other.package_id
            && self.package_id != String::new()
            && other.package_id != String::new()
    }
}

impl Eq for XMIIdLocalReference {}

impl PartialOrd for XMIIdLocalReference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for XMIIdLocalReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.package_id
            .cmp(&other.package_id)
            .then(self.object_id.cmp(&other.object_id))
    }
}

impl fmt::Debug for XMIIdLocalReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.package_id != String::new() {
            write!(
                f,
                "\"Complete XMIIdLocalReference RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else {
            write!(
                f,
                "\"Uncomplete XMIIdLocalReference RefCell of \'{}\'",
                self.object_id,
            )
        }
    }
}

impl XMIIdLocalReference {
    /// Create when only object ID is available (need to use set_package after)
    pub fn new_local(object_id: String) -> Self {
        XMIIdLocalReference {
            object_id: object_id,
            package_id: String::new(),
            is_set: false,
        }
    }

    /// Create when object ID and package ID are available (DON'T need to use set_package after)
    pub fn new_global(object_id: String, package_id: String) -> Self {
        XMIIdLocalReference {
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
    pub fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    /// Get package ID
    pub fn get_package_id(&self) -> String {
        self.package_id.clone()
    }

    /// Return combinaison of package ID and object ID
    pub fn label(&self) -> Result<String, anyhow::Error> {
        if self.is_set {
            Ok(format!(
                "{}-{}",
                self.get_package_id(),
                self.get_object_id()
            ))
        } else {
            Err(anyhow::format_err!(
                "Call \"label()\" on unset XMI ID : (XMIIdReference) {:#?}",
                self
            ))
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone)]
/// Reference to another XMI object
pub struct XMIIdReference {
    object_id: String,
    package_id: String,
    is_set: bool,
    /// Content of the ref, define with make_post_deserialize
    pub object: RefCell<Option<EnumCMOF>>,
}

impl PartialEq for XMIIdReference {
    fn eq(&self, other: &Self) -> bool {
        self.object_id == other.object_id
            && self.package_id == other.package_id
            && self.package_id != String::new()
            && other.package_id != String::new()
    }
}

impl fmt::Debug for XMIIdReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.object.borrow().is_some() {
            write!(
                f,
                "\"Loaded XMIIdReference RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else if self.package_id != String::new() {
            write!(
                f,
                "\"UnLoaded XMIIdReference RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else {
            write!(
                f,
                "\"Uncomplete XMIIdReference RefCell of \'{}\'",
                self.object_id,
            )
        }
    }
}

impl XMIIdReference {
    /// Create when only object ID is available (need to use set_package after)
    pub fn new_local(object_id: String) -> Self {
        XMIIdReference {
            object_id: object_id,
            package_id: String::new(),
            is_set: false,
            object: RefCell::new(None),
        }
    }

    /// Create when object ID and package ID are available (DON'T need to use set_package after)
    pub fn new_global(object_id: String, package_id: String) -> Self {
        XMIIdReference {
            object_id: object_id,
            package_id: package_id,
            is_set: true,
            object: RefCell::new(None),
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
    pub fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    /// Get package ID
    pub fn get_package_id(&self) -> String {
        self.package_id.clone()
    }

    /// Return combinaison of package ID and object ID
    pub fn label(&self) -> Result<String, anyhow::Error> {
        if self.is_set {
            Ok(format!(
                "{}-{}",
                self.get_package_id(),
                self.get_object_id()
            ))
        } else {
            Err(anyhow::format_err!(
                "Call \"label()\" on unset XMI ID : (XMIIdReference) {:#?}",
                self
            ))
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################

/// Tools for CMOF Object
pub trait SetCMOFTools {
    /// Allow to finish XMIId of object and collect all CMOF object
    /// Use "dict_setting" for share content between parent object to child object
    /// Use "dict_object" for collect all object
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error>;
    /// Allow to define the post-treatment method : post_deserialize
    /// Link external XMI Id of object by matching it on "dict_object"
    /// Use "dict_object" for obtain object between objects
    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error>;
}

// ####################################################################################################
//
// ####################################################################################################

/// Tool for CMOF Object
pub trait GetXMIId {
    /// Allow to get the xmi id field label (use in global BTreeMap)
    fn get_xmi_id_field(&self) -> Result<String, anyhow::Error>;
    /// Allow to get the xmi id field object name (use in local BTreeMap)
    fn get_xmi_id_object(&self) -> Result<String, anyhow::Error>;
}
