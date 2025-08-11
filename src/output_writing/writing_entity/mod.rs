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
#![doc = include_str!("../../../doc/writing_entity.md")]

// Mod section
pub mod writing_entity_for_association;
pub mod writing_entity_for_class;
pub mod writing_entity_for_datatype;
pub mod writing_entity_for_enumeration;
pub mod writing_entity_for_primitive_type;
pub mod writing_entity_for_property;
pub use writing_entity_for_association::*;
pub use writing_entity_for_class::*;
pub use writing_entity_for_datatype::*;
pub use writing_entity_for_enumeration::*;
pub use writing_entity_for_primitive_type::*;
pub use writing_entity_for_property::*;

// Package section
use crate::cmof_loader::*;
use crate::custom_log_tools::*;
use crate::output_writing::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make a module file for each pckg
    pub fn write_mod_object(&mut self) -> Result<(), anyhow::Error> {
        let enumeration_default_values = read_enumeration_default_values()?;
        let primitive_type_conversion = read_primitive_type_conversion()?;
        for (label, pckg) in self.get_package_in_order() {
            debug!("Generating sub-mod file for \"{label}\" : START");
            for (_, entity) in &pckg.get_json().owned_member {
                match entity {
                    EnumOwnedMember::Association(content) => {
                        // Only for "Many to Many"
                        let association = content.get_association_relation()?;
                        if content.need_file(association)? {
                            // Get file
                            let (_, mut wrt) = self.get_object_file(entity);
                            //
                            let r = content.write_content(&mut wrt);
                            catch_error_and_log(r, content)?
                        }
                    }
                    EnumOwnedMember::Class(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(entity);
                        //
                        let r = content.write_content(&mut wrt, &primitive_type_conversion);
                        catch_error_and_log(r, content)?
                    }
                    EnumOwnedMember::DataType(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(entity);
                        //
                        let r = content.write_content(&mut wrt, &primitive_type_conversion);
                        catch_error_and_log(r, content)?
                    }
                    EnumOwnedMember::Enumeration(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(entity);
                        //
                        let r = content.write_content(&mut wrt, &enumeration_default_values);
                        catch_error_and_log(r, content)?;
                    }
                    EnumOwnedMember::PrimitiveType(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(entity);
                        //
                        let r = content.write_content(&mut wrt, &primitive_type_conversion);
                        catch_error_and_log(r, content)?
                    }
                }
            }
            info!("Generating sub-mod file for \"{label}\" : Finished");
        }
        Ok(())
    }
}
