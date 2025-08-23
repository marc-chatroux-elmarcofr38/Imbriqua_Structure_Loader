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

impl CMOFClass {
    /// write content to output file,from "CMOFClass" object
    pub fn write_content(
        &self,
        wrt: &mut File,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        let _ = writeln!(
            wrt,
            include_str!("template/entity_main_class.tmpl"),
            full_name = self.full_name,
            import = self.get_import_content()?,
            table_name = self.table_name,
            fields = self.get_fields_content(primitive_type_conversion)?,
            relations = self.get_relation_content()?,
            related = self.get_related_content()?,
            raw = format!("{:#?}", self).prefix("// "),
        );
        Ok(())
    }

    /// "import" content for entity_class_main.tmpl
    fn get_import_content(&self) -> Result<String, anyhow::Error> {
        let mut result = String::from("\n");

        // Only for field that use CMOFEnumeration result, i.e. that have simple property without association
        let mut need_import = false;
        for property in self.get_all_simple_field()? {
            if property.simple_type.is_some() {
                need_import = true;
            }
        }
        if need_import {
            result.push_str("use crate::*;\n");
        }

        // For all
        result.push_str("use sea_orm::entity::prelude::*;\n");

        Ok(result)
    }

    /// "fields" content for entity_class_main.tmpl
    fn get_fields_content(
        &self,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::from("");

        // For super class
        for (_, class) in self.get_super_class()? {
            CMOFClass::format_field_super(&class, &mut result)?;
        }

        // For complex property
        for field in self.get_all_complex_field()? {
            CMOFClass::format_field_complex_property(
                &field,
                &mut result,
                primitive_type_conversion,
            )?;
        }

        // For simple property
        for field in self.get_all_simple_field()? {
            CMOFClass::format_field_simple_property(
                &field,
                &mut result,
                primitive_type_conversion,
            )?;
        }

        Ok(result)
    }

    /// "relations" content for entity_class_main.tmpl
    fn get_relation_content(&self) -> Result<String, anyhow::Error> {
        let mut result = String::new();

        // For direct "Super"
        for (_, class) in &self.get_super_class()? {
            let object_class = get_object_as_enum(&class)?;
            let object_class = match object_class {
                EnumCMOF::CMOFClass(c) => c,
                _ => panic!("dfghjdfghjdfgh"),
            };
            CMOFClass::format_relation_super_from_one(&self, &object_class, &mut result)?;
        }

        // For reverse "Super"
        for super_class in self.get_reverse_super_class()? {
            CMOFClass::format_relation_super_to_one(&self, &super_class, &mut result)?;
        }

        Ok(result)
    }

    /// "related" content for entity_class_main.tmpl
    fn get_related_content(&self) -> Result<String, anyhow::Error> {
        let mut result = String::new();

        // For "Super"
        for (_, class) in self.get_super_class()? {
            let object_class = get_object_as_class(class)?;
            CMOFClass::format_related_direct_super(&self, &object_class, &mut result)?;
        }

        // For reverse "Super"
        for super_class in self.get_reverse_super_class()? {
            CMOFClass::format_related_reverse_super(&self, &super_class, &mut result)?;
        }

        Ok(result)
    }

    /// Get all simple field
    fn get_all_simple_field(&self) -> Result<Vec<&CMOFProperty>, anyhow::Error> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for (_, property) in &self.owned_attribute {
            match property {
                EnumOwnedAttribute::Property(content) => {
                    if content.upper > infinitable::Finite(1) {
                        // Not a field, N-N link
                    } else if content.simple_type.is_some() {
                        if content.association.is_none() {
                            result.push(&content)
                        }
                    } else if matches!(
                        content.complex_type.as_ref().unwrap(),
                        EnumType::HRefPrimitiveType(_)
                    ) {
                        result.push(&content);
                    };
                }
            }
        }

        Ok(result)
    }

    /// Get all complex field
    fn get_all_complex_field(&self) -> Result<Vec<&CMOFProperty>, anyhow::Error> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for (_, property) in &self.owned_attribute {
            match property {
                EnumOwnedAttribute::Property(content) => {
                    if content.upper > infinitable::Finite(1) {
                        // Not a field, N-N link
                    } else if content.simple_type.is_some() {
                        if content.association.is_some() {
                            result.push(&content)
                        }
                    } else if matches!(
                        content.complex_type.as_ref().unwrap(),
                        EnumType::HRefClass(_)
                    ) {
                        result.push(&content);
                    } else if matches!(
                        content.complex_type.as_ref().unwrap(),
                        EnumType::HRefDataType(_)
                    ) {
                        result.push(&content);
                    };
                }
            }
        }

        Ok(result)
    }

    /// Format "Super" from __get_all_direct_super__, to write field part
    fn format_field_super(
        class: &XMIIdReference<EnumWeakCMOF>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let object_class = get_object_as_class(&class)?;
        // Comment
        result.push_str(
            format!(
                "    /// SUPER FIELD : {comment}\n",
                comment = &object_class.super_model_name
            )
            .as_str(),
        );
        // Pub element
        let field_name = &object_class.super_field_name;
        let field_type = String::from("i64");
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = field_name,
                field_type = field_type,
            )
            .as_str(),
        );

        Ok(())
    }

    /// Format "Simple property" from __get_all_simple_field__, to write field part
    fn format_field_simple_property(
        content: &CMOFProperty,
        result: &mut String,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        // Comment
        result.push_str(
            format!(
                "    /// SIMPLE FIELD : {comment}\n",
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
        }
        // Pub element
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = content.get_field_name(),
                field_type = content.get_field_type(primitive_type_conversion)?,
            )
            .as_str(),
        );
        Ok(())
    }

    /// Format "Complex property" from __get_all_complex_field__, to write field part
    fn format_field_complex_property(
        content: &CMOFProperty,
        result: &mut String,
        primitive_type_conversion: &PrimitiveTypeConversion,
    ) -> Result<(), anyhow::Error> {
        // Comment
        result.push_str(
            format!(
                "    /// COMPLEX FIELD : {comment}\n",
                comment = content.xmi_id.label()?
            )
            .as_str(),
        );
        // SEA_ORM element
        // Pub element
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = &content.name.to_case(Case::Snake),
                field_type = content.get_field_type(primitive_type_conversion)?,
            )
            .as_str(),
        );
        Ok(())
    }

    fn format_relation_super_from_one(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "DIRECT SUPER : ONE {} need ONE {}",
            self_class.model_name, super_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("template/entity_sub_super_relation.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
                foreign_field = super_class.super_model_name,
            )
            .as_str(),
        );
        Ok(())
    }

    fn format_relation_super_to_one(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "REVERSE SUPER : ONE {} need ONE {}",
            super_class.model_name, self_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("template/entity_sub_relation_to_one.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
            )
            .as_str(),
        );
        Ok(())
    }

    /// Format "Super" from __get_all_direct_super__, to write related part
    fn format_related_direct_super(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "DIRECT SUPER : ONE {} need ONE {}",
            self_class.model_name, super_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("template/entity_sub_super_related.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
            )
            .as_str(),
        );
        Ok(())
    }

    /// Format inverse of "Super" from __get_all_reverse_super__, to write related part
    fn format_related_reverse_super(
        self_class: &CMOFClass,
        super_class: &Rc<CMOFClass>,
        result: &mut String,
    ) -> Result<(), anyhow::Error> {
        let comment = format!(
            "REVERSE SUPER : ONE {} need ONE {}",
            super_class.model_name, self_class.model_name,
        );
        result.push_str(
            format!(
                include_str!("template/entity_sub_super_related.tmpl"),
                table_name = super_class.table_name,
                model_name = super_class.model_name,
                comment = comment,
            )
            .as_str(),
        );
        Ok(())
    }
}
