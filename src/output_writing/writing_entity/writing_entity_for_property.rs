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

impl CMOFProperty {
    pub fn get_field_type(
        &self,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::new();

        if self.upper > infinitable::Finite(1) {
            return Ok(result);
        }

        // OPTION
        result.push_str(if self.lower == 0 { "Option<" } else { "" });

        // For field simple

        let content = if self.simple_type.is_some() {
            if self.association.is_none() {
                // Simple field, i.e. other Enumeration
                let a = self.simple_type.as_ref().unwrap().get_object_as_enum()?;
                match a {
                    EnumCMOF::CMOFClass(c) => c.model_name.clone(),
                    EnumCMOF::CMOFPrimitiveType(c) => {
                        let r = primitive_type_conversion.get(&c.model_name);
                        if r.is_none() {
                            error!("{}", c.model_name);
                            String::new()
                        } else {
                            r.unwrap().clone()
                        }
                    }
                    EnumCMOF::CMOFDataType(c) => c.model_name.clone(),
                    EnumCMOF::CMOFEnumeration(c) => c.model_name.clone(),
                    _ => {
                        return Err(anyhow::format_err!(
                            "not type for \"{}\"",
                            &self.simple_type.as_ref().unwrap().label()?
                        ));
                    }
                }
            } else {
                // Foreign field
                String::from("i64")
            }
        } else {
            match self.complex_type.as_ref().unwrap() {
                EnumType::HRefPrimitiveType(link) => {
                    // Simple field
                    let c = link.href.get_object()?;
                    let c = c.upgrade();
                    let c = if c.is_some() {
                        c.unwrap()
                    } else {
                        return Err(anyhow::format_err!("errberb"));
                    };
                    let r = primitive_type_conversion.get(&c.model_name);
                    if r.is_none() {
                        error!("{}", c.model_name);
                        String::new()
                    } else {
                        r.unwrap().clone()
                    }
                }
                EnumType::HRefClass(_) => {
                    // Foreign field
                    String::from("i64")
                }
                EnumType::HRefDataType(_) => {
                    // Foreign field
                    String::from("i64")
                }
            }
        };
        result.push_str(content.as_str());

        // OPTION
        result.push_str(if self.lower == 0 { ">" } else { "" });

        Ok(result)
    }

    pub fn get_field_name(&self) -> String {
        if &self.name.to_case(Case::Snake) == &String::from("id") {
            String::from("bpmn_id")
        } else {
            self.name.to_case(Case::Snake)
        }
    }
}
