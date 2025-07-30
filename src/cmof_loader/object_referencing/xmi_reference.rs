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
/// Reference to self XMI object
pub struct XMIIdLocalReference {
    object_id: String,
    package_id: String,
}

impl PartialEq for XMIIdLocalReference {
    fn eq(&self, other: &Self) -> bool {
        self.object_id == other.object_id && self.package_id == other.package_id
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
                "Complete XMIIdLocalReference RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else {
            write!(
                f,
                "Uncomplete XMIIdLocalReference RefCell of \'{}\'",
                self.object_id,
            )
        }
    }
}

impl XMIIdLocalReference {
    /// Create when only object ID is available (need to use set_package_id after)
    pub fn new_local(object_id: String) -> Self {
        XMIIdLocalReference {
            object_id: object_id,
            package_id: String::new(),
        }
    }

    /// Create when object ID and package ID are available (DON'T need to use set_package_id after)
    pub fn new_global(object_id: String, package_id: String) -> Self {
        XMIIdLocalReference {
            object_id: object_id,
            package_id: package_id,
        }
    }

    /// Define package ID after a new_local
    pub fn set_package_id(&mut self, package_id: &String) {
        self.package_id = package_id.clone();
    }

    /// Get object ID
    pub fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    /// Get package ID
    pub fn get_package_id(&self) -> String {
        self.package_id.clone()
    }

    /// True, if package_id and object_id not empty
    pub fn is_set(&self) -> bool {
        !self.package_id.is_empty() && !self.object_id.is_empty()
    }

    /// Return combinaison of package ID and object ID
    pub fn label(&self) -> Result<String, anyhow::Error> {
        if self.is_set() {
            Ok(format!(
                "{}-{}",
                self.get_package_id(),
                self.get_object_id()
            ))
        } else {
            Err(anyhow::format_err!(
                "Call \"label()\" on unset XMI ID : (XMIIdReference<EnumWeakCMOF>) {:#?}",
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
pub struct XMIIdReference<T> {
    object_id: String,
    package_id: String,
    /// Content of the ref, define with make_post_deserialize
    object: RefCell<Option<T>>,
}

impl<T> Default for XMIIdReference<T> {
    fn default() -> Self {
        XMIIdReference {
            object_id: String::new(),
            package_id: String::new(),
            object: RefCell::new(None),
        }
    }
}

impl<T> PartialEq for XMIIdReference<T> {
    fn eq(&self, other: &Self) -> bool {
        self.object_id == other.object_id && self.package_id == other.package_id
    }
}

impl<T> Eq for XMIIdReference<T> {}

impl<T> PartialOrd for XMIIdReference<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for XMIIdReference<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.package_id
            .cmp(&other.package_id)
            .then(self.object_id.cmp(&other.object_id))
    }
}

impl<T> fmt::Debug for XMIIdReference<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.object.borrow().is_some() {
            write!(
                f,
                "Loaded XMIIdReference<EnumWeakCMOF> RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else if self.package_id != String::new() {
            write!(
                f,
                "UnLoaded XMIIdReference<EnumWeakCMOF> RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else if self.object_id != String::new() {
            write!(
                f,
                "Uncomplete XMIIdReference<EnumWeakCMOF> RefCell of \'{}\'",
                self.object_id,
            )
        } else {
            write!(f, "Empty XMIIdReference<EnumWeakCMOF> RefCell")
        }
    }
}

impl<T: Clone> XMIIdReference<T> {
    /// Create when only object ID is available (need to use set_package_id after)
    pub fn new_local(object_id: String) -> Self {
        XMIIdReference {
            object_id: object_id,
            package_id: String::new(),
            object: RefCell::new(None),
        }
    }

    /// Create when object ID and package ID are available (DON'T need to use set_package_id after)
    pub fn new_global(object_id: String, package_id: String) -> Self {
        XMIIdReference {
            object_id: object_id,
            package_id: package_id,
            object: RefCell::new(None),
        }
    }

    /// Define package ID after a new_local
    pub fn set_object_id(&mut self, object_id: &String) {
        if self.object_id.is_empty() {
            self.object_id = object_id.clone();
        }
    }

    /// Define package ID after a new_local
    pub fn set_package_id(&mut self, package_id: &String) {
        self.package_id = package_id.clone();
    }

    /// Define object content
    pub fn set_object(&self, content: T) -> Result<(), anyhow::Error> {
        self.object.replace(Some(content));
        Ok(())
    }

    /// Get object ID
    pub fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    /// Get package ID
    pub fn get_package_id(&self) -> String {
        self.package_id.clone()
    }

    /// Get object content
    pub fn get_object(&self) -> Result<T, anyhow::Error> {
        let r = self.object.borrow();
        let r = r.as_ref();
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Error in XMIIdReference<EnumWeakCMOF> 'get_object' : None result during borrow of Refcell"
            ));
        }
        Ok(r.unwrap().clone())
    }

    /// True, if package_id and object_id not empty
    pub fn is_set(&self) -> bool {
        !self.package_id.is_empty() && !self.object_id.is_empty() && self.object.borrow().is_some()
    }

    /// Return combinaison of package ID and object ID
    pub fn label(&self) -> Result<String, anyhow::Error> {
        if self.is_set() {
            Ok(format!(
                "{}-{}",
                self.get_package_id(),
                self.get_object_id()
            ))
        } else {
            Err(anyhow::format_err!(
                "Call \"label()\" on unset XMI ID : (XMIIdReference<EnumWeakCMOF>) {:#?}",
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
