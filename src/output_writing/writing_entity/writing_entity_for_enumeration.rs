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

impl CMOFEnumeration {
    /// write content to output file,from "CMOFEnumeration" object
    pub fn write_content(
        &self,
        wrt: &mut File,
        enumeration_default_values: &EnumerationDefaultValues,
    ) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            wrt,
            include_str!("template/entity_main_enumeration.tmpl"),
            full_name = self.full_name,
            model_name = self.model_name,
            fields = self.get_fields_content(enumeration_default_values)?,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    fn get_fields_content(
        &self,
        enumeration_default_values: &EnumerationDefaultValues,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::from("");

        // For all literal
        for literal in self.get_all_literal()? {
            self.write_literal_property(literal, &mut result, enumeration_default_values)?;
        }

        Ok(result)
    }

    fn write_literal_property(
        &self,
        literal: &CMOFEnumerationLiteral,
        result: &mut String,
        enumeration_default_values: &EnumerationDefaultValues,
    ) -> Result<(), anyhow::Error> {
        // Default element
        let is_default = if literal.is_default(enumeration_default_values)? {
            "\n    #[default]"
        } else {
            ""
        };

        result.push_str(
            format!(
                include_str!("template/entity_sub_enumeration_literal.tmpl"),
                comment = literal.xmi_id.label()?,
                is_default = is_default,
                enumeration_value_snake = literal.litteral_designation,
                enumeration_value_camel = literal.litteral_name,
            )
            .as_str(),
        );
        Ok(())
    }
}

impl CMOFEnumerationLiteral {
    fn is_default(
        &self,
        enumeration_default_values: &EnumerationDefaultValues,
    ) -> Result<bool, anyhow::Error> {
        match self.parent.get_object()?.upgrade()? {
            EnumCMOF::CMOFEnumeration(content) => {
                if !enumeration_default_values.contains_key(&content.model_name) {
                    Err(anyhow::format_err!(
                        "No enuneration default value for {}",
                        &content.model_name
                    ))
                } else {
                    Ok(enumeration_default_values.get(&content.model_name).unwrap()
                        == &self.litteral_name)
                }
            }
            _ => Err(anyhow::format_err!(
                "Enumeration Literal without Literal Parent {}",
                self.litteral_name
            )),
        }
    }
}
