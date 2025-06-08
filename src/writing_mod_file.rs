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
use crate::writing_manager::*;

// Dependencies section

// ####################################################################################################
//
// ########################################## MAIN ####################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make a module file for each package
    pub fn write_mod_file(&mut self) {
        for (label, package) in self.get_package_in_order() {
            // Logs
            debug!("Generating \"mod.rs\" for \"{label}\" : START");

            // Create folder and lib file
            let _ = self.get_output_mod_folder(package);
            let (_, mut writing_mod_file) = self.get_output_mod_file(package);

            // 1 - Write mod head
            package.get_json().wrt_mod_head(&mut writing_mod_file);
            // 1 - Write mod head

            // 2 - Write mod object call
            package
                .get_json()
                .wrt_mod_object_call(&mut writing_mod_file);
            // 2 - Write mod object call

            // Logs
            info!("Generating \"mod.rs\" for \"{label}\" : Finished");
        }
    }
}

// ####################################################################################################
//
// ############################################ 1 #####################################################
//
// ####################################################################################################

impl WritingModHead for CMOFPackage {
    fn wrt_mod_head(&self, writer: &mut File) {
        // Doc title
        let _ = writeln!(writer, "//! {}", self.get_lowercase_name());
        let _ = writeln!(writer, "\n#![allow(unused_imports)]");

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
        let _ = writeln!(writer, "\n/// Link from {} (PackageImport)", self.xmi_id);
        match &self.imported_package {
            EnumImportedPackage::ImportedPackage(package) => {
                let _ = writeln!(writer, "use crate::{};", package.get_level_path());
            }
        }
    }
}

// ####################################################################################################
//
// ############################################ 2 #####################################################
//
// ####################################################################################################

impl WritingModObjectCall for CMOFPackage {
    fn wrt_mod_object_call(&self, writer: &mut File) {
        for class in self.get_sorted_iter() {
            class.wrt_mod_object_call(writer)
        }
    }
}

impl WritingModObjectCall for EnumOwnedMember {
    fn wrt_mod_object_call(&self, writer: &mut File) {
        match self {
            EnumOwnedMember::Association(content) => {
                let _ = content;
                // content.wrt_mod_object_call(writer);
            }
            EnumOwnedMember::Class(content) => {
                let _ = content;
                // content.wrt_mod_object_call(writer);
            }
            EnumOwnedMember::DataType(content) => {
                let _ = content;
                // content.wrt_mod_object_call(writer);
            }
            EnumOwnedMember::Enumeration(content) => {
                content.wrt_mod_object_call(writer);
            }
            EnumOwnedMember::PrimitiveType(content) => {
                let _ = content;
                content.wrt_mod_object_call(writer);
            }
        }
    }
}

impl WritingModObjectCall for CMOFAssociation {
    fn wrt_mod_object_call(&self, writer: &mut File) {
        let _ = writeln!(writer, "\n/// Association : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_level_path(),
            self.get_level_path(),
            self.get_level_struct()
        );
    }
}

impl WritingModObjectCall for CMOFClass {
    fn wrt_mod_object_call(&self, writer: &mut File) {
        let _ = writeln!(writer, "\n/// Class : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_level_path(),
            self.get_level_path(),
            self.get_level_struct()
        );
    }
}

impl WritingModObjectCall for CMOFDataType {
    fn wrt_mod_object_call(&self, writer: &mut File) {
        let _ = writeln!(writer, "\n/// DataType : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_level_path(),
            self.get_level_path(),
            self.get_level_struct()
        );
    }
}

impl WritingModObjectCall for CMOFEnumeration {
    fn wrt_mod_object_call(&self, writer: &mut File) {
        let _ = writeln!(writer, "\n/// Enumeration : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_level_path(),
            self.get_level_path(),
            self.get_level_struct()
        );
    }
}

impl WritingModObjectCall for CMOFPrimitiveType {
    fn wrt_mod_object_call(&self, writer: &mut File) {
        let _ = writeln!(writer, "\n/// PrimitiveType : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_level_path(),
            self.get_level_path(),
            self.get_level_struct()
        );
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################
