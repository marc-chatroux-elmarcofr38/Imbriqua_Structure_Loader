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
#![doc = include_str!("../doc/module_write_lib.md")]

// Package section
use crate::module_cmof_structure::*;
use crate::module_dependencies_explorer::*;
use crate::module_log::*;

// Dependencies section
// use std::fmt::Debug;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Build all pre calculing information
    pub fn writing_preparation(&mut self) {
        // Alone classes
        for (_, (_, package)) in self.clone().get_package_in_order() {
            for owned_member in package.get_json().owned_member.clone().into_iter() {
                match owned_member {
                    EnumOwnedMember::Class(content) => {
                        let mut is_alone = true;
                        for owned_attribute in content.owned_attribute {
                            match owned_attribute {
                                EnumOwnedAttribute::Property(content_2) => {
                                    if !is_simple_dpt(content_2.name.as_str()) {
                                        is_alone = false;
                                    }
                                }
                            }
                        }
                        if is_alone {
                            self.pre_calculation
                                .class_classification
                                .insert(content.name, ClassClassification::Simple);
                        }
                    }
                    _ => {}
                }
            }
        }
        debug!(
            "Simple Class {:?}",
            self.pre_calculation.class_classification
        );
        // build_class_classification();
    }
}

/// Bullshit function : define if a type (represent as string.....) involve to set a structure using reference ("&")
pub fn is_simple_dpt(input: &str) -> bool {
    match input {
        "Boolean" => false,
        "Integer" => false,
        "Real" => false,
        "String" => false,
        "i8" => false,
        "u8" => false,
        "dc::Boolean" => false,
        "dc::Integer" => false,
        "dc::Real" => false,
        "dc::String" => false,
        _ => true,
    }
}
