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
use crate::module_dependencies_explorer::*;
use crate::module_file_manager::*;
use crate::module_log::*;

// Dependencies section
use std::fmt::Debug;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Get output file
    fn get_output_lib_file(&self) -> (String, File) {
        // Calculate folder path
        let mut file_name = self.get_output_folder();
        let filename = "lib.rs";
        file_name.push(filename);
        // Create empty file
        let writer = file_name.write_new_file();
        (filename.to_string(), writer)
    }
    /// Make lib.rs from scratch and package
    pub fn write_lib(&mut self) {
        let (filename, mut writing_file) = self.get_output_lib_file();
        // Write head
        let _ = write!(
            writing_file,
            "#![doc = include_str!(\"../README.md\")]\n\n//! \n\n//! Imported from {:?}\n\n",
            self.get_output_folder()
        );
        // Write body
        for (_, (label, package)) in self.get_package_in_order() {
            // Logs
            debug!("Generating \"{filename}\" from \"{label}\" : START");
            // Write lib head
            package.wrt_lib_level(&mut writing_file);
            // Logs
            info!("Generating \"{filename}\" from \"{label}\" : Finished");
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

/// Implement writing of target lib loading element as Rust
pub trait WritingLibHead: Debug {
    /// Implement writing of target lib loading element as Rust
    fn wrt_lib_level(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

impl WritingLibHead for LoadingPackage {
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
