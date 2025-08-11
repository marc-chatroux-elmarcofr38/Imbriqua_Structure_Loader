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

//! Provide referencing link between object of CMOF file (referenced by XMI)
//!
//! eefefef
//!
//! # Content :
//!
//! * [XMIIdLocalReference] : Structure for saving key of an XMI object
//! * [XMIIdReference] : Structure  for saving call to an other XMI object in a XMI object
//! * [SetCMOFTools] : Trait
//!     * [SetCMOFTools::collect_object]
//!     * [SetCMOFTools::make_post_deserialize]
//!
//! # Example :
//!
//! zagdhfj

// ####################################################################################################
//
// ####################################################################################################

// Package section
// use crate::cmof_loader::*;

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
    /// Define package ID after a new_local, but don't change if not empty
    pub fn set_package_id_if_empty(&mut self, package_id: &String) {
        if self.package_id.is_empty() {
            self.package_id = package_id.clone();
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
                "Loaded XMIIdReference RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else if self.package_id != String::new() {
            write!(
                f,
                "UnLoaded XMIIdReference RefCell of \'{}-{}\'",
                self.package_id, self.object_id,
            )
        } else if self.object_id != String::new() {
            write!(
                f,
                "Uncomplete XMIIdReference RefCell of \'{}\'",
                self.object_id,
            )
        } else {
            write!(f, "Empty XMIIdReference RefCell")
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

    /// Define object ID after a new_local
    pub fn set_object_id(&mut self, object_id: &String) {
        self.object_id = object_id.clone();
    }

    /// Define object ID after a new_local, but don't change if not empty
    pub fn set_object_id_if_empty(&mut self, object_id: &String) {
        if self.package_id.is_empty() {
            self.object_id = object_id.clone();
        }
    }

    /// Get object ID
    pub fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    /// Define package ID after a new_local
    pub fn set_package_id(&mut self, package_id: &String) {
        self.package_id = package_id.clone();
    }

    /// Define package ID after a new_local, but don't change if not empty
    pub fn set_package_id_if_empty(&mut self, package_id: &String) {
        if self.package_id.is_empty() {
            self.package_id = package_id.clone();
        }
    }

    /// Get package ID
    pub fn get_package_id(&self) -> String {
        self.package_id.clone()
    }

    /// Define object content
    pub fn set_object(&self, content: T) {
        self.object.replace(Some(content));
    }

    /// Define object content, but don't change if not empty
    pub fn set_object_if_empty(&self, content: T) {
        if self.object.borrow().is_none() {
            self.object.replace(Some(content));
        }
    }

    /// Get object content
    pub fn get_object(&self) -> Result<T, anyhow::Error> {
        let r = self.object.borrow();
        let r = r.as_ref();
        if r.is_none() {
            return Err(anyhow::format_err!(
                "Error in XMIIdReference 'get_object' : None result during borrow of Refcell"
            ));
        }
        Ok(r.unwrap().clone())
    }

    /// True, if package_id and object_id not empty
    pub fn is_set(&self) -> bool {
        !self.package_id.is_empty() && !self.object_id.is_empty()
    }

    /// True, if package_id and object_id not empty, and object if not None
    pub fn is_set_and_complete(&self) -> bool {
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
                "Call \"label()\" on unset XMI ID : (XMIIdReference) {:#?}",
                self
            ))
        }
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
    fn local_reference_01_creation() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let _ = XMIIdLocalReference::new_local(String::from("object_1"));
            let _ = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_02_eq_and_partial_eq() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_1"),
            );
            let ref_3 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_2"),
            );
            let ref_4 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );

            let ref_1_bis = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            // Equality
            assert!(ref_1 == ref_1);
            assert!(ref_2 == ref_2);
            assert!(ref_3 == ref_3);
            assert!(ref_4 == ref_4);

            assert!(ref_1 == ref_1_bis);

            // Non-equality
            assert!(ref_1 != ref_2);
            assert!(ref_1 != ref_3);
            assert!(ref_1 != ref_4);
            assert!(ref_2 != ref_3);
            assert!(ref_2 != ref_4);
            assert!(ref_3 != ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_03_ord_and_partial_ord() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_1"),
            );
            let ref_3 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_2"),
            );
            let ref_4 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );

            // Inequality
            assert!(ref_1 < ref_2);
            assert!(ref_1 < ref_3);
            assert!(ref_1 < ref_4);
            assert!(ref_2 < ref_3);
            assert!(ref_2 < ref_4);
            assert!(ref_3 < ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_04_debug() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1 = XMIIdLocalReference::new_local(String::from("object_1"));
            assert_eq!(
                format!("{:?}", ref_1),
                String::from("Uncomplete XMIIdLocalReference RefCell of \'object_1\'")
            );

            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_2"),
                String::from("package_2"),
            );
            assert_eq!(
                format!("{:?}", ref_2),
                String::from("Complete XMIIdLocalReference RefCell of \'package_2-object_2\'")
            );

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_05_set_package_id() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1: XMIIdLocalReference =
                XMIIdLocalReference::new_local(String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            assert_eq!(ref_1, ref_2);

            let mut ref_1: XMIIdLocalReference =
                XMIIdLocalReference::new_local(String::from("object_1"));
            ref_1.set_package_id_if_empty(&String::from("package_1"));
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            assert_eq!(ref_1, ref_2);

            ref_1.set_package_id_if_empty(&String::from("other_content"));

            assert_eq!(ref_1, ref_2);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_06_get_fields() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1 = XMIIdLocalReference::new_local(String::from("object_1"));
            ref_1.set_package_id_if_empty(&String::from("package_1"));
            let ref_2 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            assert_eq!(ref_1.get_object_id(), String::from("object_1"));
            assert_eq!(ref_1.get_package_id(), String::from("package_1"));
            assert_eq!(ref_2.get_object_id(), String::from("object_1"));
            assert_eq!(ref_2.get_package_id(), String::from("package_1"));

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn local_reference_07_label_and_is_set() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1 = XMIIdLocalReference::new_global(
                String::from("object_1"),
                String::from("package_1"),
            );

            assert!(ref_1.is_set());
            assert!(ref_1.label().is_ok());
            assert_eq!(ref_1.label().unwrap(), String::from("package_1-object_1"));

            let ref_2 = XMIIdLocalReference::new_local(String::from("object_1"));

            assert!(!ref_2.is_set());
            assert!(ref_2.label().is_err());

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_01_creation() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let _: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));
            let _: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));
            let _: XMIIdReference<String> = XMIIdReference::default();

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_02_eq_and_partial_eq() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));
            let ref_2: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_1"));
            let ref_3: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_2"));
            let ref_4 =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            let ref_1_bis =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            // Equality
            assert!(ref_1 == ref_1);
            assert!(ref_2 == ref_2);
            assert!(ref_3 == ref_3);
            assert!(ref_4 == ref_4);

            assert!(ref_1 == ref_1_bis);

            // Non-equality
            assert!(ref_1 != ref_2);
            assert!(ref_1 != ref_3);
            assert!(ref_1 != ref_4);
            assert!(ref_2 != ref_3);
            assert!(ref_2 != ref_4);
            assert!(ref_3 != ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_03_ord_and_partial_ord() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            // package is more important that object

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));
            let ref_2: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_1"));
            let ref_3: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_2"));
            let ref_4: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            // Inequality
            assert!(ref_1 < ref_2);
            assert!(ref_1 < ref_3);
            assert!(ref_1 < ref_4);
            assert!(ref_2 < ref_3);
            assert!(ref_2 < ref_4);
            assert!(ref_3 < ref_4);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_04_debug() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> = XMIIdReference::default();
            assert_eq!(
                format!("{:?}", ref_1),
                String::from("Empty XMIIdReference RefCell")
            );

            let ref_2: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));
            assert_eq!(
                format!("{:?}", ref_2),
                String::from("Uncomplete XMIIdReference RefCell of \'object_1\'")
            );

            let ref_3: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));
            assert_eq!(
                format!("{:?}", ref_3),
                String::from("UnLoaded XMIIdReference RefCell of \'package_2-object_2\'")
            );

            let ref_4: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));
            let content = String::from("CONTENT !!!");
            ref_4.set_object(content);
            assert_eq!(
                format!("{:?}", ref_4),
                String::from("Loaded XMIIdReference RefCell of \'package_2-object_2\'")
            );

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_05_set_package_id() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1: XMIIdReference<String> =
                XMIIdReference::new_local(String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2 =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert_eq!(ref_1, ref_2);

            let mut ref_1: XMIIdReference<String> =
                XMIIdReference::new_local(String::from("object_1"));
            ref_1.set_package_id_if_empty(&String::from("package_1"));
            let ref_2 =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert_eq!(ref_1, ref_2);

            ref_1.set_package_id_if_empty(&String::from("other_content"));

            assert_eq!(ref_1, ref_2);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_06_set_object_id() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1: XMIIdReference<String> = XMIIdReference::default();
            ref_1.set_object_id(&String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2 =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert_eq!(ref_1, ref_2);

            let mut ref_1: XMIIdReference<String> = XMIIdReference::default();
            ref_1.set_object_id_if_empty(&String::from("object_1"));
            ref_1.set_package_id(&String::from("package_1"));
            let ref_2 =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert_eq!(ref_1, ref_2);

            ref_1.set_object_id_if_empty(&String::from("other_content"));

            assert_eq!(ref_1, ref_2);

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_07_get_fields() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let mut ref_1: XMIIdReference<String> =
                XMIIdReference::new_local(String::from("object_1"));
            ref_1.set_package_id_if_empty(&String::from("package_1"));
            let ref_2: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert_eq!(ref_1.get_object_id(), String::from("object_1"));
            assert_eq!(ref_1.get_package_id(), String::from("package_1"));
            assert_eq!(ref_2.get_object_id(), String::from("object_1"));
            assert_eq!(ref_2.get_package_id(), String::from("package_1"));

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_08_set_object() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            assert!(ref_1.is_set());
            assert!(!ref_1.is_set_and_complete());

            let content = String::from("CONTENT !!!");
            ref_1.set_object(content);

            assert!(ref_1.is_set());
            assert!(ref_1.is_set_and_complete());

            let content = String::from("OTHER CONTENT !!!");
            ref_1.set_object_if_empty(content);

            assert!(ref_1.is_set());
            assert!(ref_1.is_set_and_complete());

            assert_eq!(ref_1.get_object()?, String::from("CONTENT !!!"));

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_09_get_object() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_2"), String::from("package_2"));

            assert!(ref_1.is_set());
            assert!(!ref_1.is_set_and_complete());
            assert!(ref_1.get_object().is_err());

            let content = String::from("CONTENT !!!");
            ref_1.set_object(content);

            assert!(ref_1.is_set());
            assert!(ref_1.is_set_and_complete());
            assert_eq!(ref_1.get_object()?, String::from("CONTENT !!!"));

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }

    #[test]
    fn reference_10_label() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            let ref_1: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));

            assert!(ref_1.is_set());
            assert!(!ref_1.is_set_and_complete());
            assert!(ref_1.label().is_ok());

            let ref_2: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));

            assert!(!ref_2.is_set());
            assert!(!ref_2.is_set_and_complete());
            assert!(ref_2.label().is_err());

            let ref_3: XMIIdReference<String> = XMIIdReference::new_local(String::from("object_1"));
            let content = String::from("CONTENT !!!");
            ref_3.set_object(content);

            assert!(!ref_3.is_set());
            assert!(!ref_3.is_set_and_complete());
            assert!(ref_3.label().is_err());

            let ref_4: XMIIdReference<String> =
                XMIIdReference::new_global(String::from("object_1"), String::from("package_1"));
            let content = String::from("CONTENT !!!");
            ref_4.set_object(content);

            assert!(ref_4.is_set());
            assert!(ref_4.is_set_and_complete());
            assert!(ref_4.label().is_ok());
            assert_eq!(ref_4.label().unwrap(), String::from("package_1-object_1"));
            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }
}
