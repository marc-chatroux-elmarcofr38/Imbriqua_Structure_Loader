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
pub use infinitable::Infinitable as UnlimitedNatural;
use std::collections::BTreeMap;

// ####################################################################################################
//
// ####################################################################################################

/// __False__, as default value for serde_default
pub fn default_false() -> bool {
    false
}

/// __True__, as default value for serde_default
pub fn default_true() -> bool {
    true
}

// ####################################################################################################
//
// ####################################################################################################

/// Empty String, as default value for serde_default
pub fn default_lower() -> i32 {
    1
}

/// Empty String, as default value for serde_default
pub fn default_upper() -> UnlimitedNatural<i32> {
    infinitable::Finite(1)
}

// ####################################################################################################
//
// ####################################################################################################

/// Empty String, as default value for serde_default
pub fn default_empty_string() -> String {
    String::new()
}

/// Empty Vec, as default value for serde_default
pub fn default_empty_vec<T>() -> Vec<T> {
    Vec::new()
}

/// Empty Vec, as default value for serde_default
pub fn default_empty_btreemap<K, V>() -> BTreeMap<K, V> {
    BTreeMap::new()
}

// ####################################################################################################
//
// ####################################################################################################

/// Default VisibilityKind, as default value for serde_default
pub fn default_visibility() -> UMLVisibilityKind {
    UMLVisibilityKind::Public
}

// ####################################################################################################
//
// ####################################################################################################

/// Default VisibilityKind, as default value for serde_default
pub fn default_option<T>() -> Option<T> {
    None
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn test_01_full_check_default_false() {
        initialize_log_for_test();

        assert_eq!(default_false(), false);
    }

    #[test]
    fn test_02_full_check_default_true() {
        initialize_log_for_test();

        assert_eq!(default_true(), true);
    }

    #[test]
    fn test_03_full_check_default_lower() {
        initialize_log_for_test();

        assert_eq!(default_lower(), 1);
    }

    #[test]
    fn test_04_full_check_default_upper() {
        initialize_log_for_test();

        assert_eq!(default_upper(), infinitable::Finite(1));
    }

    #[test]
    fn test_05_full_check_default_empty_string() {
        initialize_log_for_test();

        assert_eq!(default_empty_string(), String::new());
    }

    #[test]
    fn test_06_full_check_default_empty_vec() {
        initialize_log_for_test();

        let r1: Vec<i32> = default_empty_vec();
        let r2: Vec<i32> = Vec::new();

        assert_eq!(r1, r2);

        let r1: Vec<String> = default_empty_vec();
        let r2: Vec<String> = Vec::new();

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_07_full_check_default_empty_btreemap() {
        initialize_log_for_test();

        let r1: BTreeMap<String, i32> = default_empty_btreemap();
        let r2: BTreeMap<String, i32> = BTreeMap::new();

        assert_eq!(r1, r2);

        let r1: BTreeMap<i32, String> = default_empty_btreemap();
        let r2: BTreeMap<i32, String> = BTreeMap::new();

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_08_full_check_default_visibility() {
        initialize_log_for_test();

        assert_eq!(default_visibility(), UMLVisibilityKind::Public)
    }

    #[test]
    fn test_09_full_check_default_option() {
        initialize_log_for_test();

        let r1: Option<String> = default_option();

        assert_eq!(r1, None);

        let r1: Option<i32> = default_option();

        assert_eq!(r1, None);
    }
}
