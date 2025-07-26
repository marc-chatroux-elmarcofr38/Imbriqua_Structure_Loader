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
pub fn default_lower() -> isize {
    1
}

/// Empty String, as default value for serde_default
pub fn default_upper() -> UnlimitedNatural<usize> {
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
