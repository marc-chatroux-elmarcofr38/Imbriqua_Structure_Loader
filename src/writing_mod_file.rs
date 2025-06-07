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
#![doc = include_str!("../doc/writing_mod_file.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_cmof_structure::*;
use crate::loader_dependencies_explorer::*;

// Dependencies section
use lazy_static::lazy_static;
pub use serde_json;
use std::collections::HashMap;
use std::fmt::Debug;

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make a module file for each package
    pub fn write_mod_file(&mut self) {
        for (label, package) in self.get_package_in_order() {
            // Logs
            debug!("Generating mod.rs for \"{label}\" : START");
            // Create folder and lib file
            let folder = self.get_output_mod_folder(package);
            let (_, mut writing_mod_file) = self.get_output_package_mod_file(package);
            // Write mod head
            package.get_json().wrt_mod_head(&mut writing_mod_file);
            // Logs
            info!("Generating mod.rs for \"{label}\" : Finished");
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

/// Implement writing of target mod loading head element as Rust
pub trait WritingModHead: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : module file head (import part, "use", etc.)
    fn wrt_mod_head(&self, writer: &mut File) {
        let _ = writeln!(writer);
        let _ = write!(writer, "{}", format!("{:#?}", self).prefix("// "));
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModHead for CMOFPackage {
    fn wrt_mod_head(&self, writer: &mut File) {
        // Doc title
        let _ = writeln!(writer, "//! {}", self.get_lowercase_name());
        let _ = writeln!(writer, "use derive_builder::Builder;");

        // Import
        for import in self.package_import.iter() {
            match import {
                EnumPackageImport::PackageImport(content) => {
                    content.wrt_mod_head(writer);
                }
            }
        }
    }
}

impl WritingModHead for CMOFPackageImport {
    fn wrt_mod_head(&self, writer: &mut File) {
        let _ = writeln!(writer, "/// Link from {} (PackageImport)", self.xmi_id);
        match &self.imported_package {
            EnumImportedPackage::ImportedPackage(package) => {
                let content = package.href.clone();
                let content = content.replace(".cmof#_0", "");
                let content = content.to_case(Case::Snake);
                let _ = writeln!(writer, "use crate::{};", content);
            }
        }
    }
}
