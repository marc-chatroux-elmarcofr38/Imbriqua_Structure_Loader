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

impl CMOFDataType {
    /// write content to output file,from "CMOFDataType" object
    pub fn write_content(
        &self,
        wrt: &mut File,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            wrt,
            include_str!("template/entity_main_datatype.tmpl"),
            full_name = self.full_name,
            table_name = self.table_name,
            fields = self.get_fields_content(primitive_type_conversion)?,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    fn get_fields_content(
        &self,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::from("");

        // For all property
        for field in self.get_all_field()? {
            let content = CMOFDataType::write_field_property(&field, primitive_type_conversion)?;
            result.push_str(content.as_str());
        }

        Ok(result)
    }

    fn get_all_field(&self) -> Result<Vec<&CMOFProperty>, anyhow::Error> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for (_, property) in &self.owned_attribute {
            match property {
                EnumOwnedAttribute::Property(content) => {
                    result.push(&content);
                }
            }
        }

        Ok(result)
    }

    fn write_field_property(
        content: &CMOFProperty,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<String, anyhow::Error> {
        let mut result: String = String::new();

        // Comment
        result.push_str(
            format!(
                "    /// RUST DATA TYPE : {comment}\n",
                comment = content.xmi_id.label()?
            )
            .as_str(),
        );
        // SEA_ORM element
        if content.default.is_some() {
            result.push_str(
                format!(
                    "    #[sea_orm(default_value = \"{default_value}\")]\n",
                    default_value = content.default.as_ref().unwrap()
                )
                .as_str(),
            );
        };
        // Pub element
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = &content.name.to_case(Case::Snake),
                field_type = content.get_field_type(primitive_type_conversion)?,
            )
            .as_str(),
        );
        Ok(result)
    }
}
