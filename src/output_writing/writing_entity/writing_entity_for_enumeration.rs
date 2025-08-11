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
            include_str!("../template/entity_main_enumeration.tmpl"),
            full_name = self.full_name,
            model_name = self.model_name,
            fields = self.get_fields_content(enumeration_default_values)?,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    /// "fields" content for entity_enumeration_main.tmpl
    fn get_fields_content(
        &self,
        enumeration_default_values: &EnumerationDefaultValues,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::from("");

        // For all literal
        for literal in self.get_all_literal() {
            CMOFEnumeration::write_literal_property(
                literal,
                &self,
                &mut result,
                enumeration_default_values,
            )?;
        }

        Ok(result)
    }

    /// Get all literal
    fn get_all_literal(&self) -> Vec<&CMOFEnumerationLiteral> {
        // As default, empty
        let mut result: Vec<&CMOFEnumerationLiteral> = Vec::new();

        for (_, property) in &self.owned_attribute {
            match property {
                EnumOwnedLiteral::EnumerationLiteral(content) => {
                    result.push(&content);
                }
            }
        }

        result
    }

    // Write content for a literal
    fn write_literal_property(
        literal: &CMOFEnumerationLiteral,
        enumeration: &CMOFEnumeration,
        result: &mut String,
        enumeration_default_values: &EnumerationDefaultValues,
    ) -> Result<(), anyhow::Error> {
        // Comment
        result.push_str(
            format!(
                "    /// ENUMERATION LITERAL : {comment}\n",
                comment = literal.xmi_id.label()?
            )
            .as_str(),
        );
        // Default element
        if enumeration_default_values.contains_key(&enumeration.model_name) {
            let value_1 = enumeration_default_values
                .get(&enumeration.model_name)
                .unwrap();
            let value_2 = &literal.litteral_name;
            if value_1 == value_2 {
                //
                result.push_str(format!("    #[default]\n",).as_str());
            }
        } else {
            warn!(
                "No enuneration default value for {}",
                enumeration.model_name
            )
        };
        // Pub element
        result.push_str(
            format!(
                "    #[sea_orm(string_value = \"{enumeration_value_snake}\")]\n",
                enumeration_value_snake = literal.litteral_designation,
            )
            .as_str(),
        );
        result.push_str(
            format!(
                "    {enumeration_value_camel},\n",
                enumeration_value_camel = literal.litteral_name,
            )
            .as_str(),
        );
        Ok(())
    }
}
