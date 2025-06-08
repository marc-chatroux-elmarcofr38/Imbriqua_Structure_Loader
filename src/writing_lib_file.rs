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
#![doc = include_str!("../doc/writing_lib_file.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_dependencies_explorer::*;

// Dependencies section
use std::fmt::Debug;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make lib.rs from scratch and package
    pub fn write_lib_file(&mut self) {
        let (filename, mut writing_file) = self.get_output_lib_file();
        // Write head
        let _ = write!(
            writing_file,
            "#![doc = include_str!(\"../README.md\")]\n\n//! Imported from {:?}\n\n",
            self.get_output_folder()
        );
        let _ = writeln!(writing_file, "pub use derive_builder::Builder;\n\n");
        // Write body
        for (label, package) in self.get_package_in_order() {
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
trait WritingLibHead: Debug {
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
