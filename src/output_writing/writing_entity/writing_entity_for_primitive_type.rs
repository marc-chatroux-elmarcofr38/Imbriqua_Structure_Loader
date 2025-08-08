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

// Mod section
use crate::output_writing::writing_entity::*;

// Package section
use crate::cmof_loader::*;
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

// Dependencies section

// // ####################################################################################################
// //
// // ####################################################################################################

impl CMOFPrimitiveType {
    /// write content to output file,from "CMOFPrimitiveType" object
    pub fn write_content(
        &self,
        wrt: &mut File,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        let object_type = self.model_name.as_str();
        if primitive_type_conversion.get(object_type).is_some() {
            let content = primitive_type_conversion.get(object_type).unwrap();
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_main_primitive_type.tmpl"),
                full_name = self.full_name,
                model_name = self.model_name,
                standard_object = content,
            );
        }
        Ok(())
    }
}
