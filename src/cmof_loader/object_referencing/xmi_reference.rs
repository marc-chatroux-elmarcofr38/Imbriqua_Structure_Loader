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

#[derive(Clone, Debug)]
/// Reference to another XMI object
pub struct XMIIdLocalReference {
    object_id: String,
    package_id: String,
    is_set: bool,
}

impl PartialEq for XMIIdLocalReference {
    fn eq(&self, other: &Self) -> bool {
        self.label() == other.label()
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
    pub fn label(&self) -> String {
        if self.is_set {
            format!("{}-{}", self.get_package_id(), self.get_object_id())
        } else {
            panic!(
                "Call \"label()\" on unset XMI ID : {}",
                self.get_object_id()
            );
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
        self.label() == other.label()
    }
}

impl fmt::Debug for XMIIdReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"RefCell of \'{}\' (loaded : {})\"",
            self.label(),
            self.object.borrow().is_some(),
        )
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
    pub fn label(&self) -> String {
        if self.is_set {
            format!("{}-{}", self.get_package_id(), self.get_object_id())
        } else {
            panic!(
                "Call \"label()\" on unset XMI ID : {}",
                self.get_object_id()
            );
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
    fn get_xmi_id_field(&self) -> String;
    /// Allow to get the xmi id field object name (use in local BTreeMap)
    fn get_xmi_id_object(&self) -> String;
}

// ####################################################################################################
//
// ####################################################################################################

/// Naming method for providing struct name
pub trait NamingStruct {
    /// --> DC.cmof#Font
    fn get_technical_name(&self, package: &LoadingPackage) -> String;
    /// --> dc_font
    fn get_table_name(&self, package: &LoadingPackage) -> String;
    /// --> Font
    fn get_model_name(&self) -> String;
    /// --> dc_datatype_font
    fn get_full_name(&self, package: &LoadingPackage) -> String;
}
