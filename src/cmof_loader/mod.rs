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
#![doc = include_str!("cmof_loader.md")]

// Mod section
pub mod association_treatment;
pub mod cmof_object;
pub mod deserialize_helper;
pub mod loading_tools;
pub mod object_referencing;
pub use association_treatment::*;
pub use cmof_object::*;
pub use deserialize_helper::*;
pub use loading_tools::*;
pub use object_referencing::*;

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::output_result_manager::*;

// Dependencies section
pub use serde::Deserialize;
pub use std::collections::BTreeMap;
pub use std::rc::{Rc, Weak};

// ####################################################################################################
//
// ####################################################################################################

/// Shorcut of __LoadingTracker::new()__, creating LoadingTracker instance using ResultEnv object
pub fn open_loader(file_env: ResultEnv) -> Result<LoadingTracker, anyhow::Error> {
    LoadingTracker::new(file_env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn loader_dependencies_explorer_01_open_loader() {
        // Logs
        initialize_log_for_test();
        // Setting
        let input_folder =
            "tests/loader_dependencies_explorer/loader_dependencies_explorer_01_open_loader/input";
        let main_output_folder =
            "tests/loader_dependencies_explorer/loader_dependencies_explorer_01_open_loader/output";
        let result_folder =
            "tests/loader_dependencies_explorer/loader_dependencies_explorer_01_open_loader/result";
        // Preparing
        let file_env = open_env(input_folder, main_output_folder, result_folder).unwrap();
        // Test
        let loading_env = open_loader(file_env).unwrap();
        let _ = loading_env.get_output_folder();
    }
}
