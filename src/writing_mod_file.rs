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
            // Get folder and file
            self.get_package_folder(package);
            let (filename, mut writer) = self.get_package_mod_file(package);

            // Logs
            debug!(
                "Generating \"{}\" from \"{}\" : START",
                filename.display(),
                label
            );

            // 1 - Write mod head
            package
                .get_json()
                .wrt_mod_file_head(&mut writer, &self.pre_calculation);
            // 1 - Write mod head

            // 2 - Write mod object call
            for owned_member in package.get_json().get_sorted_iter() {
                match owned_member {
                    EnumOwnedMember::Association(_content) => {
                        // _content.wrt_mod_file_object_section(&mut writer, &self.pre_calculation);
                    }
                    EnumOwnedMember::Class(_content) => {
                        _content.wrt_mod_file_object_section(&mut writer, &self.pre_calculation);
                    }
                    EnumOwnedMember::DataType(_content) => {
                        _content.wrt_mod_file_object_section(&mut writer, &self.pre_calculation);
                    }
                    EnumOwnedMember::Enumeration(_content) => {
                        _content.wrt_mod_file_object_section(&mut writer, &self.pre_calculation);
                    }
                    EnumOwnedMember::PrimitiveType(_content) => {
                        _content.wrt_mod_file_object_section(&mut writer, &self.pre_calculation);
                    }
                }
            }
            // 2 - Write mod object call

            // Logs
            info!(
                "Generating \"{}\" from \"{}\" : Finished",
                filename.display(),
                label
            );
        }
    }
}

// ####################################################################################################
//
// ############################################ 1 #####################################################
//
// ####################################################################################################

impl WritingModFileHead for CMOFPackage {
    fn wrt_mod_file_head(&self, writer: &mut File, _pre_calculation: &LoadingPreCalculation) {
        // Doc title
        let _ = writeln!(writer, "//! {}", self.get_lowercase_name());
        let _ = writeln!(writer, "\n#![allow(unused_imports)]");

        // Import
        for import in self.package_import.iter() {
            match import {
                EnumPackageImport::PackageImport(content) => {
                    content.wrt_mod_file_head(writer, _pre_calculation);
                }
            }
        }
    }
}

impl WritingModFileHead for CMOFPackageImport {
    fn wrt_mod_file_head(&self, writer: &mut File, _pre_calculation: &LoadingPreCalculation) {
        let _ = writeln!(writer, "\n/// Link from {} (PackageImport)", self.xmi_id);
        match &self.imported_package {
            EnumImportedPackage::ImportedPackage(package) => {
                let _ = writeln!(writer, "use crate::{};", package.get_path_name());
            }
        }
    }
}

// ####################################################################################################
//
// ############################################ 2 #####################################################
//
// ####################################################################################################

impl WritingModFileObjectSection for CMOFAssociation {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// Association : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_path_name(),
            self.get_path_name(),
            self.get_struct_name()
        );
    }
}

impl WritingModFileObjectSection for CMOFClass {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// Class : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_path_name(),
            self.get_path_name(),
            self.get_struct_name()
        );
    }
}

impl WritingModFileObjectSection for CMOFDataType {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// DataType : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_path_name(),
            self.get_path_name(),
            self.get_struct_name()
        );
    }
}

impl WritingModFileObjectSection for CMOFEnumeration {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// Enumeration : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_path_name(),
            self.get_path_name(),
            self.get_struct_name()
        );
    }
}

impl WritingModFileObjectSection for CMOFPrimitiveType {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// PrimitiveType : {}", self.name);
        let _ = writeln!(
            writer,
            "mod {};\npub use {}::{};",
            self.get_path_name(),
            self.get_path_name(),
            self.get_struct_name()
        );
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################
