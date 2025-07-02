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
// ####################################################################################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Make a module file for each pckg
    pub fn write_mod_object(&mut self) {
        for (label, pckg) in self.get_package_in_order() {
            debug!("Generating sub-mod file for \"{label}\" : START");
            for entity in pckg.get_sorted_iter() {
                match entity {
                    EnumOwnedMember::Association(content) => {
                        // Only for "Many to Many"
                        let association =
                            self.pre_calculation.association_relation.get(&content.name);
                        if association.is_some()
                            && association.unwrap()[0].upper == infinitable::Infinity
                            && association.unwrap()[1].upper == infinitable::Infinity
                        {
                            // Get file
                            let (_, mut wrt) = self.get_object_file(pckg, entity);
                            //
                            content.write_content(&mut wrt, &pckg, &self.pre_calculation);
                        }
                    }
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
                        content.write_content(&mut wrt, &pckg, &self.pre_calculation);
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
                        content.write_content(&mut wrt, &pckg, &self.pre_calculation);
                    }
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

impl CMOFAssociation {
    /// write content to output file,from "CMOFClass" object
    fn write_content(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_main_association.tmpl"),
            full_name = self.get_full_name(pckg),
            import = self.get_import_content(pckg, pre_calc),
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
            include_str!("../template/entity_main_class.tmpl"),
            full_name = self.get_full_name(pckg),
            import = self.get_import_content(pckg, pre_calc),
            table_name = self.get_table_name(pckg),
            fields = self.get_fields_content(pckg, pre_calc),
            relations = self.get_relations_content(pckg, pre_calc),
            related = self.get_related_content(pckg, pre_calc),
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
    fn get_relations_content(&self, _pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        // For direct "Super"
        for super_field in self.get_all_super() {
            let field_name = super_field
                .to_case(Case::Snake)
                .prefix("super_")
                .replace("\n", "");
            let key = super_field;
            if pre_calc.owned_member_type_list.contains_key(&key) {
                let matched_named = pre_calc.owned_member_type_list.get(&key).unwrap();
                let table_name = &matched_named.table_name;
                let model_name = &matched_named.model_name;
                let comment = format!(
                    "SUPER : ONE {} need ONE {}",
                    self.get_model_name(),
                    model_name
                );
                result.push_str(
                    format!(
                        include_str!("../template/entity_sub_relation_many.tmpl"),
                        table_name = table_name,
                        model_name = model_name,
                        comment = comment,
                        foreign_field = field_name.to_case(Case::UpperCamel),
                    )
                    .as_str(),
                );
            }
        }

        // For reverse "Super"
        if pre_calc
            .reverse_super_link
            .contains_key(&self.get_model_name())
        {
            let reverse_super = pre_calc
                .reverse_super_link
                .get(&self.get_model_name())
                .unwrap();
            for super_field in reverse_super {
                let key = super_field;
                if pre_calc.owned_member_type_list.contains_key(key) {
                    let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
                    let table_name = &matched_named.table_name;
                    let model_name = &matched_named.model_name;
                    let comment = format!(
                        "SUPER : ONE {} need ONE {}",
                        model_name,
                        self.get_model_name()
                    );
                    result.push_str(
                        format!(
                            include_str!("../template/entity_sub_relation_one.tmpl"),
                            table_name = table_name,
                            model_name = model_name,
                            comment = comment,
                        )
                        .as_str(),
                    );
                }
            }
        }

        result
    }

    /// "related" content for entity_class_main.tmpl
    fn get_related_content(&self, _pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        // For "Super"
        for super_field in self.get_all_super() {
            let key = super_field;
            if pre_calc.owned_member_type_list.contains_key(&key) {
                let matched_named = pre_calc.owned_member_type_list.get(&key).unwrap();
                let table_name = &matched_named.table_name;
                let model_name = &matched_named.model_name;
                let comment = format!(
                    "SUPER : ONE {} need ONE {}",
                    self.get_model_name(),
                    model_name
                );
                result.push_str(
                    format!(
                        include_str!("../template/entity_sub_related.tmpl"),
                        table_name = table_name,
                        model_name = model_name,
                        comment = comment,
                    )
                    .as_str(),
                );
            }
        }

        // For reverse "Super"
        if pre_calc
            .reverse_super_link
            .contains_key(&self.get_model_name())
        {
            let reverse_super = pre_calc
                .reverse_super_link
                .get(&self.get_model_name())
                .unwrap();
            for super_field in reverse_super {
                let key = super_field;
                if pre_calc.owned_member_type_list.contains_key(key) {
                    let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
                    let table_name = &matched_named.table_name;
                    let model_name = &matched_named.model_name;
                    let comment = format!(
                        "SUPER : ONE {} need ONE {}",
                        model_name,
                        self.get_model_name()
                    );
                    result.push_str(
                        format!(
                            include_str!("../template/entity_sub_related.tmpl"),
                            table_name = table_name,
                            model_name = model_name,
                            comment = comment,
                        )
                        .as_str(),
                    );
                }
            }
        }

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
        result.push_str(format!("    /// SUPER FIELD : {comment}\n", comment = class).as_str());
        // Pub element
        let field_name = class
            .to_case(Case::Snake)
            .prefix("super_")
            .replace("\n", "");
        let field_type = String::from("i32");
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
        let field_name = if &content.name.to_case(Case::Snake) == &String::from("id") {
            &String::from("bpmn_id")
        } else {
            &content.name.to_case(Case::Snake)
        };
        result.push_str(
            format!(
                "    pub {field_name}: {field_type},\n",
                field_name = field_name,
                field_type = content.get_field_type(&pre_calc),
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
                field_type = content.get_field_type(&pre_calc),
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

impl CMOFDataType {
    /// write content to output file,from "CMOFDataType" object
    fn write_content(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_main_datatype.tmpl"),
            full_name = self.get_full_name(pckg),
            table_name = self.get_table_name(pckg),
            fields = self.get_fields_content(pckg, pre_calc),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }
    /// "fields" content for entity_data_type_main.tmpl
    fn get_fields_content(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::from("");

        // For all property
        for field in self.get_all_field() {
            CMOFDataType::write_field_property(&field, &mut result, pckg, pre_calc);
        }

        result
    }
    /// Get all field
    fn get_all_field(&self) -> Vec<&CMOFProperty> {
        // As default, empty
        let mut result: Vec<&CMOFProperty> = Vec::new();

        for property in &self.owned_attribute {
            match property {
                EnumOwnedAttribute::Property(content) => {
                    result.push(&content);
                }
            }
        }

        result
    }

    // Write field content
    fn write_field_property(
        content: &CMOFProperty,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        // Comment
        result.push_str(
            format!(
                "    /// RUST DATA TYPE : {comment}\n",
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
                field_type = content.get_field_type(&pre_calc),
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

impl CMOFEnumeration {
    /// write content to output file,from "CMOFEnumeration" object
    fn write_content(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        let _ = writeln!(
            wrt,
            include_str!("../template/entity_main_enumeration.tmpl"),
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

impl CMOFPrimitiveType {
    /// write content to output file,from "CMOFPrimitiveType" object
    fn write_content(&self, wrt: &mut File, pckg: &LPckg, pre_calc: &LPreCalc) {
        let object_type = self.name.as_str();
        if pre_calc
            .primitive_type_conversion
            .get(object_type)
            .is_some()
        {
            let content = pre_calc.primitive_type_conversion.get(object_type).unwrap();
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_main_primitive_type.tmpl"),
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
    fn get_field_type(&self, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        if self.upper > infinitable::Finite(1) {
            return result;
        }

        // OPTION
        result.push_str(if self.lower == 0 { "Option<" } else { "" });

        // For field simple

        let content = if self.simple_type.is_some() {
            if self.association.is_none() {
                // Simple field
                self.simple_type.as_ref().unwrap().as_str()
            } else {
                // Foreign field
                "i32"
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
                        "i32"
                    }
                }
                EnumType::ClassLink(_) => {
                    // Foreign field
                    "i32"
                }
                EnumType::DataTypeLink(_) => {
                    // Foreign field
                    "i32"
                }
            }
        };
        result.push_str(content);

        // OPTION
        result.push_str(if self.lower == 0 { ">" } else { "" });

        result
    }
}
