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
use crate::loader_cmof_structure::*;
use crate::loader_dependencies_explorer::*;
use crate::writing_manager::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make lib.rs from scratch and package
    pub fn write_lib_file(&mut self) {
        // Get folder and file
        let (filename, mut writer) = self.get_project_lib_file();
        let _ = writeln!(writer, "\n/// Imported from {:?}", self.get_output_folder());

        for (label, package) in self.get_package_in_order() {
            // Logs
            debug!(
                "Generating \"{}\" from \"{}\" : START",
                filename.display(),
                label
            );

            // 1 - Write mod object call
            for owned_member in package.get_sorted_iter() {
                match owned_member {
                    EnumOwnedMember::Association(_content) => {}
                    EnumOwnedMember::Class(_content) => {
                        _content.wrt_mod_file_object_section(
                            &mut writer,
                            &package,
                            &self.pre_calculation,
                        );
                    }
                    EnumOwnedMember::DataType(_content) => {
                        _content.wrt_mod_file_object_section(
                            &mut writer,
                            &package,
                            &self.pre_calculation,
                        );
                    }
                    EnumOwnedMember::Enumeration(_content) => {
                        _content.wrt_mod_file_object_section(
                            &mut writer,
                            &package,
                            &self.pre_calculation,
                        );
                    }
                    EnumOwnedMember::PrimitiveType(_content) => {}
                }
            }
            // 1 - Write mod object call

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

impl WritingModFileObjectSection for CMOFClass {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        package: &LoadingPackage,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// Class : {}", self.name);
        let _ = writeln!(writer, "pub mod {};", self.get_path_name(package),);
    }
}

impl WritingModFileObjectSection for CMOFDataType {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        package: &LoadingPackage,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// DataType : {}", self.name);
        let _ = writeln!(writer, "pub mod {};", self.get_path_name(package),);
    }
}

impl WritingModFileObjectSection for CMOFEnumeration {
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        package: &LoadingPackage,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(writer, "\n/// Enumeration : {}", self.name);
        let _ = writeln!(writer, "pub mod {};", self.get_path_name(package),);
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################
