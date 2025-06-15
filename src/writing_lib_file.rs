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

        // Head
        let _ = writeln!(
            writer,
            include_str!("../template/lib_part_1_common.tmpl"),
            folder_name = self.get_output_folder(),
        );

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
                    EnumOwnedMember::Class(content) => {
                        content.wrt_lib_file_level(&mut writer, &package, &self.pre_calculation);
                    }
                    EnumOwnedMember::DataType(content) => {
                        content.wrt_lib_file_level(&mut writer, &package, &self.pre_calculation);
                    }
                    _ => {}
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

impl WritingLibFile for CMOFClass {
    fn wrt_lib_file_level(
        &self,
        writer: &mut File,
        package: &LoadingPackage,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(
            writer,
            include_str!("../template/lib_part_2_class.tmpl"),
            entity_name = self.name,
            entity_file_name = self.get_table_name(package),
        );
    }
}

impl WritingLibFile for CMOFDataType {
    fn wrt_lib_file_level(
        &self,
        writer: &mut File,
        package: &LoadingPackage,
        _pre_calculation: &LoadingPreCalculation,
    ) {
        let _ = writeln!(
            writer,
            include_str!("../template/lib_part_2_datatype.tmpl"),
            entity_name = self.name,
            entity_file_name = self.get_table_name(package),
        );
    }
}
