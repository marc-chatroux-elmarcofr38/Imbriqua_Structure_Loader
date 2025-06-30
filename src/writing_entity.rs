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
#![doc = include_str!("../doc/writing_entity.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_cmof_structure::*;
use crate::loader_dependencies_explorer::LoadingPackage as LPckg;
use crate::loader_dependencies_explorer::LoadingPreCalculation as LPreCalc;
use crate::loader_dependencies_explorer::*;
use crate::writing_manager::*;

// Dependencies section

// ####################################################################################################
//
// ########################################## MAIN ####################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make a module file for each pckg
    pub fn write_mod_object(&mut self) {
        for (label, pckg) in self.get_package_in_order() {
            debug!("Generating sub-mod file for \"{label}\" : START");
            for entity in pckg.get_sorted_iter() {
                match entity {
                    EnumOwnedMember::Class(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.write_content(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    EnumOwnedMember::DataType(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.wrt_entity_fields_caller(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    EnumOwnedMember::Enumeration(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.write_content(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    EnumOwnedMember::PrimitiveType(content) => {
                        // Get file
                        let (_, mut wrt) = self.get_object_file(pckg, entity);
                        //
                        content.wrt_entity_fields_caller(&mut wrt, &pckg, &self.pre_calculation);
                    }
                    _ => {}
                }
            }
            info!("Generating sub-mod file for \"{label}\" : Finished");
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFClass {
    /// write content to output file,from "CMOFClass" object
    fn write_content(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_class_main.tmpl"),
            full_name = self.get_full_name(pckg),
            import = self.get_import_content(pckg, pre_calc),
            table_name = self.get_table_name(pckg),
            fields = self.get_fields_content(pckg, pre_calc),
            relations = self.get_relations_content(pckg, pre_calc),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }

    /// "import" content for entity_class_main.tmpl
    fn get_import_content(&self, _pckg: &LPckg, _pre_calc: &LPreCalc) -> String {
        let mut result = String::from("\n");
        result.push_str("use crate::*;\n");
        result.push_str("use sea_orm::entity::prelude::*;\n");
        result
    }

    /// "fields" content for entity_class_main.tmpl
    fn get_fields_content(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::from("");

        // For super class
        for class in self.get_all_super() {
            CMOFClass::write_field_super(&class, &mut result, pckg, pre_calc);
        }

        // For complex property
        for field in self.get_all_complex_field() {
            CMOFClass::write_field_complex_property(&field, &mut result, pckg, pre_calc);
        }

        // For simple property
        for field in self.get_all_simple_field() {
            CMOFClass::write_field_simple_property(&field, &mut result, pckg, pre_calc);
        }

        result
    }

    /// "relations" content for entity_class_main.tmpl
    fn get_relations_content(&self, _pckg: &LPckg, _pre_calc: &LPreCalc) -> String {
        let result = String::from("");
        result
    }

    /// Get all "Super" name
    fn get_all_super(&self) -> Vec<String> {
        // As default, empty
        let mut result: Vec<String> = self.super_class.clone();

        // // For super class link
        for link in self.super_class_link.clone() {
            match link {
                EnumSuperClass::Class(content) => {
                    let class = content.href.clone();
                    let class = match class.find(".cmof#") {
                        Some(split_index) => class[split_index..].replace(".cmof#", "").to_string(),
                        None => class,
                    };
                    result.push(class);
                }
            }
        }

        result
    }

    /// Write "Super" from __get_all_super__
    fn write_field_super(class: &String, result: &mut String, _pckg: &LPckg, _pre_calc: &LPreCalc) {
        // Comment
        result.push_str(format!("    /// SIMPLE FIELD : {comment}\n", comment = class).as_str());
        // Pub element
        let field_name = class
            .to_case(Case::Snake)
            .prefix("super_")
            .replace("\n", "");
        let field_type = String::from("i64");
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = field_name,
                field_type = field_type,
            )
            .as_str(),
        );
    }

    /// Get all simple field
    fn get_all_simple_field(&self) -> Vec<&CMOFProperty> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for property in &self.owned_attribute {
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
                        EnumType::PrimitiveTypeLink(_)
                    ) {
                        result.push(&content);
                    };
                }
            }
        }

        result
    }

    // Write field content for a simple field
    fn write_field_simple_property(
        content: &CMOFProperty,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        // Comment
        result.push_str(
            format!(
                "    /// SIMPLE FIELD : {comment}\n",
                comment = content.xmi_id
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
                field_name = &content.name.to_case(Case::Snake),
                field_type = content.get_type(&pre_calc),
            )
            .as_str(),
        );
    }

    /// Get all simple field
    fn get_all_complex_field(&self) -> Vec<&CMOFProperty> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for property in &self.owned_attribute {
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
                        EnumType::ClassLink(_)
                    ) {
                        result.push(&content);
                    } else if matches!(
                        content.complex_type.as_ref().unwrap(),
                        EnumType::DataTypeLink(_)
                    ) {
                        result.push(&content);
                    };
                }
            }
        }

        result
    }

    // Write field content for a complex field
    fn write_field_complex_property(
        content: &CMOFProperty,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        // Comment
        result.push_str(
            format!(
                "    /// COMPLEX FIELD : {comment}\n",
                comment = content.xmi_id
            )
            .as_str(),
        );
        // SEA_ORM element
        // Pub element
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = &content.name.to_case(Case::Snake),
                field_type = content.get_type(&pre_calc),
            )
            .as_str(),
        );
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFDataType {}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFEnumeration {
    /// write content to output file,from "CMOFEnumeration" object
    fn write_content(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_enumeration_main.tmpl"),
            full_name = self.get_full_name(pckg),
            model_name = self.get_model_name(),
            fields = self.get_fields_content(pckg, pre_calc),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }

    /// "fields" content for entity_enumeration_main.tmpl
    fn get_fields_content(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::from("");

        // For all literal
        for literal in self.get_all_literal() {
            CMOFEnumeration::write_literal_property(literal, &self, &mut result, pckg, pre_calc);
        }

        result
    }

    /// Get all literal
    fn get_all_literal(&self) -> Vec<&CMOFEnumerationLiteral> {
        // As default, empty
        let mut result: Vec<&CMOFEnumerationLiteral> = Vec::new();

        for property in &self.owned_attribute {
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
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        // Comment
        result.push_str(
            format!(
                "    /// ENUMERATION LITERAL : {comment}\n",
                comment = literal.xmi_id
            )
            .as_str(),
        );
        // Default element
        if pre_calc
            .enumeration_default_value
            .contains_key(&enumeration.name)
        {
            let value_1 = pre_calc
                .enumeration_default_value
                .get(&enumeration.name)
                .unwrap();
            let value_2 = &literal.name.to_case(Case::UpperCamel);
            if value_1 == value_2 {
                //
                result.push_str(format!("    #[default]\n",).as_str());
            }
        } else {
            warn!("No enuneration default value for {}", enumeration.name)
        };
        // Pub element
        result.push_str(
            format!(
                "    #[sea_orm(string_value = \"{enumeration_value_snake}\")]\n",
                enumeration_value_snake = literal.name,
            )
            .as_str(),
        );
        result.push_str(
            format!(
                "    {enumeration_value_camel},\n",
                enumeration_value_camel = literal.name.to_case(Case::UpperCamel),
            )
            .as_str(),
        );
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFPrimitiveType {}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl WritingModObjectCaller for CMOFDataType {
    fn wrt_entity_fields_caller(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        // Part 1 : Head
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_datatype_part_1.tmpl"),
            full_name = self.get_full_name(pckg),
        );
        if true {
            // for import
            let _ = writeln!(wrt, include_str!("../template/entity_datatype_part_2.tmpl"),);
        }
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_datatype_part_3.tmpl"),
            table_name = self.get_table_name(pckg),
        );

        // // Part 2 : Fields
        for field in self.owned_attribute.iter() {
            match field {
                EnumOwnedAttribute::Property(content) => {
                    content.wrt_property(wrt, pckg, pre_calc);
                }
            }
        }

        // Part 3 : End
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_datatype_part_4.tmpl"),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }
}

impl WritingModObjectCaller for CMOFPrimitiveType {
    fn wrt_entity_fields_caller(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        // Part 1 : Head
        let object_type = self.name.as_str();
        if pre_calc
            .primitive_type_conversion
            .get(object_type)
            .is_some()
        {
            let content = pre_calc.primitive_type_conversion.get(object_type).unwrap();
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_primitive_type_part_1.tmpl"),
                full_name = self.get_full_name(pckg),
                model_name = self.get_model_name(),
                standard_object = content,
            );
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFProperty {
    fn wrt_property(&self, wrt: &mut File, _pckg: &LPckg, pre_calc: &LPreCalc) {
        // type
        let name = &self.name.to_case(Case::Snake);

        if false {
            // if need of #[sea_orm ....]
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_field_part_1.tmpl"),
                field_head = "",
            );
        };

        if self.is_field() {
            let field_name = name;
            let field_type = self.get_type(&pre_calc);
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_field_part_2.tmpl"),
                comment = self.xmi_id,
                field_name = field_name,
                field_type = field_type,
            );
        }
    }
}

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

// ####################################################################################################
//
// ####################################################################################################
//
// ####################################################################################################

impl CMOFProperty {
    /// If is Foreign field or simple field
    fn is_field(&self) -> bool {
        // upper : 1 or 0
        self.upper <= infinitable::Finite(1)
    }

    /// If need to use option
    fn is_option(&self) -> bool {
        self.lower == 0
    }

    ///
    fn _is_foreign(&self, _pre_calc: &LPreCalc) -> bool {
        if !self.is_field() {
            return false;
        }

        if self.simple_type.is_some() {
            !self.association.is_none()
        } else {
            match self.complex_type.as_ref().unwrap() {
                EnumType::PrimitiveTypeLink(_) => false,
                _ => true,
            }
        }
    }

    ///
    fn get_type(&self, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        if !self.is_field() {
            return result;
        }

        // OPTION
        result.push_str(if self.is_option() { "Option<" } else { "" });

        // For field simple

        let content = if self.simple_type.is_some() {
            if self.association.is_none() {
                // Simple field
                self.simple_type.as_ref().unwrap().as_str()
            } else {
                // Foreign field
                "i64"
            }
        } else {
            match self.complex_type.as_ref().unwrap() {
                EnumType::PrimitiveTypeLink(link) => {
                    // Simple field
                    let key = link.href.clone();
                    let key = match key.find(".cmof#") {
                        Some(split_index) => key[split_index..].replace(".cmof#", "").to_string(),
                        None => key,
                    };

                    if pre_calc.primitive_type_conversion.get(&key).is_some() {
                        pre_calc.primitive_type_conversion.get(&key).unwrap()
                    } else {
                        info!("Error : unknow PRIMITIVE TYPE{}", key);
                        "i64"
                    }
                }
                EnumType::ClassLink(_) => {
                    // Foreign field
                    "i64"
                }
                EnumType::DataTypeLink(_) => {
                    // Foreign field
                    "i64"
                }
            }
        };
        result.push_str(content);

        // OPTION
        result.push_str(if self.is_option() { ">" } else { "" });

        result
    }
}
