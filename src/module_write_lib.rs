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

#![warn(missing_docs)]
#![doc = include_str!("../doc/module_write_lib.md")]

// Package section
use crate::module_dependencies_explorer::LoadingPackage;
use crate::module_dependencies_explorer::LoadingTracker;
use crate::module_file_manager::*;
use crate::module_log::*;

// Dependencies section
use std::fmt::Debug;

/// Implement writing of target lib loading element as Rust
pub trait WritingLib: Debug {
    /// Implement writing of target lib loading element as Rust
    fn wrt_lib_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

impl WritingLib for LoadingPackage {
    fn wrt_lib_level(&self, writer: &mut File) {
        // Module pachage uri
        let _ = writeln!(
            writer,
            "/// {}{} : {}",
            &self.get_json().name,
            &self.get_json().xmi_id,
            &self.get_json().uri
        );
        // Add mod import in main
        let _ = writeln!(writer, "pub mod {};", &self.get_lowercase_name());
    }
}

impl LoadingTracker {
    /// Get output file
    fn get_output_file(&self) -> File {
        // Calculate folder path
        let mut file_name = self.get_output_folder();
        file_name.push("lib.rs");
        // Create empty file
        let mut writer = file_name.write_new_file();
        writer
    }
    ///
    fn trs(&self) {}
    /// Make lib.rs from scratch and package
    pub fn make_lib_file_from_package(&mut self) {
        let mut writing_file = self.get_output_file();
        // Write head
        let _ = write!(
            writing_file,
            "#![doc = include_str!(\"../README.md\")]\n\n//! \n\n//! Imported from {:?}\n\n",
            self.get_output_folder()
        );
        // Write body
        for (key, (label, package)) in self.get_package_in_order() {
            // Logs
            debug!("Generating \"lib.rs\" from \"{}\" : START", label);
            //
            package.wrt_lib_level(&mut writing_file);
            // Logs
            info!("Generating \"lib.rs\" from \"{}\" : Finished", label);
        }
    }
}
