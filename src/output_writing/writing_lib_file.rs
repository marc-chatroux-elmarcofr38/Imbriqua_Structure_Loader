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
#![doc = include_str!("../../doc/writing_lib_file.md")]

// Package section
use crate::cmof_loader::*;
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

use crate::output_writing::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make lib.rs from scratch and package
    pub fn write_lib_file(&mut self) -> Result<(), anyhow::Error> {
        // Get folder and file
        let (_, mut writer) = self.get_project_lib_file();

        // Part 1 : Head of lib.rs, using template
        let _ = writeln!(
            writer,
            include_str!("../../template/lib_part_1_common.tmpl"),
            folder_name = self.get_output_folder(),
        );

        // Part 2 : entities part of lib.rs
        for (label, package) in self.get_package_in_order() {
            // Logs
            debug!("Generating \"lib.rs\" from \"{label}\" : START",);

            // Writting for each entities, using template
            for (_, entity) in &package.get_json().owned_member {
                match entity {
                    EnumOwnedMember::Association(content) => {
                        let t = content.get_xmi_id_field()?;
                        trace!("Start : 'write_lib_file' for {}", t);
                        content.wrt_lib_file_level(&mut writer)?;
                        trace!("End : 'write_lib_file' for {}", t);
                    }
                    EnumOwnedMember::Class(content) => {
                        let t = content.get_xmi_id_field()?;
                        trace!("Start : 'write_lib_file' for {}", t);
                        content.wrt_lib_file_level(&mut writer)?;
                        trace!("End : 'write_lib_file' for {}", t);
                    }
                    EnumOwnedMember::DataType(content) => {
                        let t = content.get_xmi_id_field()?;
                        trace!("Start : 'write_lib_file' for {}", t);
                        content.wrt_lib_file_level(&mut writer)?;
                        trace!("End : 'write_lib_file' for {}", t);
                    }
                    EnumOwnedMember::Enumeration(content) => {
                        let t = content.get_xmi_id_field()?;
                        trace!("Start : 'write_lib_file' for {}", t);
                        content.wrt_lib_file_level(&mut writer)?;
                        trace!("End : 'write_lib_file' for {}", t);
                    }
                    EnumOwnedMember::PrimitiveType(content) => {
                        let t = content.get_xmi_id_field()?;
                        trace!("Start : 'write_lib_file' for {}", t);
                        content.wrt_lib_file_level(&mut writer)?;
                        trace!("End : 'write_lib_file' for {}", t);
                    }
                }
            }

            // Logs
            info!("Generating \"lib.rs\" from \"{label}\" : Finished");
        }
        Ok(())
    }
}

// ####################################################################################################
//
// ############################################ 1 #####################################################
//
// ####################################################################################################

impl WritingLibFile for CMOFAssociation {
    fn wrt_lib_file_level(&self, writer: &mut File) -> Result<(), anyhow::Error> {
        // Only for "Many to Many"
        let association = self.get_association_relation();
        if association.ponteration_type == RelationPonderationType::ManyToMany {
            if !association.is_self_referencing {
                let _ = writeln!(
                    writer,
                    include_str!("../../template/lib_part_2_association.tmpl"),
                    model_name = self.model_name,
                    table_name = self.table_name,
                );
            } else {
                warn!(
                    "Need association lib implement for \"{}\" because it's referencin itself",
                    self.model_name
                )
            }
        };
        Ok(())
    }
}

impl WritingLibFile for CMOFClass {
    fn wrt_lib_file_level(&self, writer: &mut File) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            writer,
            include_str!("../../template/lib_part_2_class.tmpl"),
            model_name = self.model_name,
            table_name = self.table_name,
        );
        Ok(())
    }
}

impl WritingLibFile for CMOFDataType {
    fn wrt_lib_file_level(&self, writer: &mut File) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            writer,
            include_str!("../../template/lib_part_2_datatype.tmpl"),
            model_name = self.model_name,
            table_name = self.table_name,
        );
        Ok(())
    }
}

impl WritingLibFile for CMOFEnumeration {
    fn wrt_lib_file_level(&self, writer: &mut File) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            writer,
            include_str!("../../template/lib_part_2_enumeration.tmpl"),
            model_name = self.model_name,
            table_name = self.table_name,
        );
        Ok(())
    }
}

impl WritingLibFile for CMOFPrimitiveType {
    fn wrt_lib_file_level(&self, writer: &mut File) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            writer,
            include_str!("../../template/lib_part_2_primitive_type.tmpl"),
            model_name = self.model_name,
            table_name = self.table_name,
        );
        Ok(())
    }
}
