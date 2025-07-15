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
            for (_, entity) in pckg.get_sorted_owned_member() {
                match entity {
                    EnumOwnedMember::Association(content) => {
                        // Only for "Many to Many"
                        let association =
                            self.pre_calculation.association_relation.get(&content.name);
                        if association.is_some()
                            && association.unwrap().ponteration_type
                                == RelationPonderationType::ManyToMany
                        {
                            if association.unwrap().relation_1.element_type
                                != association.unwrap().relation_2.element_type
                            {
                                // Get file
                                let (_, mut wrt) = self.get_object_file(pckg, entity);
                                //
                                content.write_content(&mut wrt, &pckg, &self.pre_calculation);
                            } else {
                                warn!("Need association file implement for \"{}\" because it's referencin itself", content.name)
                            }
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
        let association = pre_calc.association_relation.get(&self.name);
        if association.is_some()
            && association.unwrap().ponteration_type == RelationPonderationType::ManyToMany
        {
            // Get association
            let content = association.unwrap();
            // Get relation 1 content
            let relation_1_named = pre_calc
                .owned_member_type_list
                .get(&content.relation_1.element_type);
            let relation_1_table_name = if relation_1_named.is_some() {
                relation_1_named.unwrap().table_name.clone()
            } else {
                "".to_string()
            };
            let relation_1_column_name_camel = &content.relation_1.element_type;
            // Get relation 2 content
            let relation_2_named = pre_calc
                .owned_member_type_list
                .get(&content.relation_2.element_type);
            let relation_2_table_name = if relation_2_named.is_some() {
                relation_2_named.unwrap().table_name.clone()
            } else {
                "".to_string()
            };
            let relation_2_column_name_camel = &content.relation_2.element_type;
            let _ = writeln!(
                wrt,
                include_str!("../template/entity_main_association.tmpl"),
                full_name = self.get_full_name(pckg),
                import = self.get_import_content(pckg, pre_calc),
                table_name = self.get_table_name(pckg),
                relation_1_table_name = relation_1_table_name,
                relation_2_table_name = relation_2_table_name,
                relation_1_column_name_snake = relation_1_column_name_camel.to_case(Case::Snake),
                relation_2_column_name_snake = relation_2_column_name_camel.to_case(Case::Snake),
                relation_1_column_name_camel = relation_1_column_name_camel,
                relation_2_column_name_camel = relation_2_column_name_camel,
                raw = format!("{:#?}", self).prefix("// "),
            );
        }
    }

    /// "import" content for entity_class_main.tmpl
    fn get_import_content(&self, _pckg: &LPckg, _pre_calc: &LPreCalc) -> String {
        let mut result = String::from("\n");
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
            relations = self.get_relation_content(pckg, pre_calc),
            related = self.get_related_content(pckg, pre_calc),
            help_doc = self.get_help(pckg, pre_calc).prefix("    /// "),
            help_fn = self.get_help(pckg, pre_calc),
            raw = format!("{:#?}", self).prefix("// "),
        );
    }

    /// "import" content for entity_class_main.tmpl
    fn get_import_content(&self, _pckg: &LPckg, _pre_calc: &LPreCalc) -> String {
        let mut result = String::from("\n");

        // Only for field that use CMOFEnumeration result, i.e. that have simple property without association
        let mut need_import = false;
        for property in self.get_all_simple_field() {
            if property.simple_type.is_some() {
                need_import = true;
            }
        }
        if need_import {
            result.push_str("use crate::*;\n");
        }

        // For all
        result.push_str("use sea_orm::entity::prelude::*;\n");

        result
    }

    /// "fields" content for entity_class_main.tmpl
    fn get_fields_content(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::from("");

        // For super class
        for class in self.get_all_direct_super() {
            CMOFClass::format_field_super(&class, &mut result, pckg, pre_calc);
        }

        // For complex property
        for field in self.get_all_complex_field() {
            CMOFClass::format_field_complex_property(&field, &mut result, pckg, pre_calc);
        }

        // For simple property
        for field in self.get_all_simple_field() {
            CMOFClass::format_field_simple_property(&field, &mut result, pckg, pre_calc);
        }

        result
    }

    /// "relations" content for entity_class_main.tmpl
    fn get_relation_content(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        // For direct "Super"
        for super_name in &self.get_all_direct_super() {
            let class_model_name = &self.get_model_name();
            let field_name = &super_name
                .to_case(Case::Snake)
                .prefix("super_")
                .replace("\n", "")
                .to_case(Case::UpperCamel);
            CMOFClass::format_relation_super_from_one(
                class_model_name,
                super_name,
                field_name,
                &mut result,
                pckg,
                pre_calc,
            );
        }

        // For reverse "Super"
        for super_name in &self.get_all_reverse_super(pre_calc) {
            let class_model_name: &String = &self.get_model_name();
            CMOFClass::format_relation_super_to_one(
                class_model_name,
                super_name,
                &mut result,
                pckg,
                pre_calc,
            );
        }

        // // For "One to One"
        // for (association_name, association) in &self.get_all_direct_one_to_one(pre_calc) {
        //     CMOFClass::format_relation_from_one(
        //         association,
        //         association_name,
        //         &mut result,
        //         pckg,
        //         pre_calc,
        //     );
        // }

        // // For "To One"
        // for (association_name, association) in &self.get_all_to_one(pre_calc) {
        //     CMOFClass::format_relation_to_one(
        //         association,
        //         association_name,
        //         &mut result,
        //         pckg,
        //         pre_calc,
        //     );
        // }

        // // For "From Many"
        // for (association_name, association) in &self.get_all_to_one(pre_calc) {
        //     CMOFClass::format_relation_from_many(
        //         association,
        //         association_name,
        //         &mut result,
        //         pckg,
        //         pre_calc,
        //     );
        // }

        result
    }

    /// "related" content for entity_class_main.tmpl
    fn get_related_content(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        // For "Super"
        for super_name in &self.get_all_direct_super() {
            let class_model_name: &String = &self.get_model_name();
            CMOFClass::format_related_direct_super(
                class_model_name,
                super_name,
                &mut result,
                pckg,
                pre_calc,
            );
        }

        // For reverse "Super"
        for super_name in &self.get_all_reverse_super(pre_calc) {
            let class_model_name: &String = &self.get_model_name();
            CMOFClass::format_related_reverse_super(
                class_model_name,
                super_name,
                &mut result,
                pckg,
                pre_calc,
            );
        }

        // For "Many To Many"
        for (association_name, actual_relation, other_relation) in
            &self.get_all_many_to_many(pre_calc)
        {
            if actual_relation.element_type != other_relation.element_type {
                CMOFClass::format_related_many_to_many(
                    association_name,
                    actual_relation,
                    other_relation,
                    &mut result,
                    pckg,
                    pre_calc,
                );
            } else {
                warn!(
                    "Need \"Many to  Many\" implement for \"{}\" linked to itself",
                    actual_relation.element_type
                );
            }
        }

        result
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

    /// Get all complex field
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

    /// Get all "Super" name
    fn get_all_direct_super(&self) -> Vec<String> {
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

        result.sort_by(|a, b| a.cmp(&b));
        result
    }

    /// Get all "Super" name
    fn get_all_reverse_super(&self, pre_calc: &LPreCalc) -> Vec<String> {
        if pre_calc
            .reverse_super_link
            .contains_key(&self.get_model_name())
        {
            let mut result = pre_calc
                .reverse_super_link
                .get(&self.get_model_name())
                .unwrap()
                .clone();

            result.sort_by(|a, b| a.cmp(&b));
            result
        } else {
            Vec::new()
        }
    }

    /// Get all Man to Many relation of the class
    fn get_all_many_to_many(
        &self,
        pre_calc: &LPreCalc,
    ) -> Vec<(String, ElementRelation, ElementRelation)> {
        let mut result: Vec<(String, ElementRelation, ElementRelation)> = Vec::new();

        let key = &self.get_model_name();
        for (association_name, association) in &pre_calc.association_relation {
            if key == &association.relation_1.element_type {
                match association.ponteration_type {
                    RelationPonderationType::ManyToMany => {
                        result.push((
                            association_name.clone(),
                            association.relation_1.clone(),
                            association.relation_2.clone(),
                        ));
                    }
                    _ => {}
                }
            }
            if key == &association.relation_2.element_type {
                match association.ponteration_type {
                    RelationPonderationType::ManyToMany => {
                        result.push((
                            association_name.clone(),
                            association.relation_2.clone(),
                            association.relation_1.clone(),
                        ));
                    }
                    _ => {}
                }
            }
        }

        result.sort_by(|(a, _, _), (b, _, _)| a.cmp(&b));
        result
    }

    /// Get all direct One To One relation of the class
    fn get_all_direct_one_to_one(&self, pre_calc: &LPreCalc) -> Vec<(String, AssociationRelation)> {
        let mut result: Vec<(String, AssociationRelation)> = Vec::new();

        let key = &self.get_model_name();
        for (association_name, association) in &pre_calc.association_relation {
            if key == &association.relation_1.element_type {
                match association.ponteration_type {
                    RelationPonderationType::OneToOne => {
                        result.push((association_name.clone(), association.clone()));
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Get all reverse One To One relation of the class
    fn get_all_reverse_one_to_one(
        &self,
        pre_calc: &LPreCalc,
    ) -> Vec<(String, AssociationRelation)> {
        let mut result: Vec<(String, AssociationRelation)> = Vec::new();

        let key = &self.get_model_name();
        for (association_name, association) in &pre_calc.association_relation {
            if key == &association.relation_2.element_type {
                match association.ponteration_type {
                    RelationPonderationType::OneToOne => {
                        result.push((association_name.clone(), association.clone()));
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Get all direct One To Many relation of the class
    fn get_all_direct_one_to_many(
        &self,
        pre_calc: &LPreCalc,
    ) -> Vec<(String, AssociationRelation)> {
        let mut result: Vec<(String, AssociationRelation)> = Vec::new();

        let key = &self.get_model_name();
        for (association_name, association) in &pre_calc.association_relation {
            if key == &association.relation_1.element_type {
                match association.ponteration_type {
                    RelationPonderationType::OneToMany => {
                        result.push((association_name.clone(), association.clone()));
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Get all reverse One To Many relation of the class
    fn get_all_reverse_one_to_many(
        &self,
        pre_calc: &LPreCalc,
    ) -> Vec<(String, AssociationRelation)> {
        let mut result: Vec<(String, AssociationRelation)> = Vec::new();

        let key = &self.get_model_name();
        for (association_name, association) in &pre_calc.association_relation {
            if key == &association.relation_2.element_type {
                match association.ponteration_type {
                    RelationPonderationType::OneToMany => {
                        result.push((association_name.clone(), association.clone()));
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Get all direct Many To Many relation of the class
    fn get_all_direct_many_to_many(
        &self,
        pre_calc: &LPreCalc,
    ) -> Vec<(String, AssociationRelation)> {
        let mut result: Vec<(String, AssociationRelation)> = Vec::new();

        let key = &self.get_model_name();
        for (association_name, association) in &pre_calc.association_relation {
            if key == &association.relation_1.element_type {
                match association.ponteration_type {
                    RelationPonderationType::ManyToMany => {
                        result.push((association_name.clone(), association.clone()));
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Get all reverse Many To Many relation of the class
    fn get_all_reverse_many_to_many(
        &self,
        pre_calc: &LPreCalc,
    ) -> Vec<(String, AssociationRelation)> {
        let mut result: Vec<(String, AssociationRelation)> = Vec::new();

        let key = &self.get_model_name();
        for (association_name, association) in &pre_calc.association_relation {
            if key == &association.relation_2.element_type {
                match association.ponteration_type {
                    RelationPonderationType::ManyToMany => {
                        result.push((association_name.clone(), association.clone()));
                    }
                    _ => {}
                }
            }
        }

        result
    }

    /// Format "Super" from __get_all_direct_super__, to write field part
    fn format_field_super(
        class: &String,
        result: &mut String,
        _pckg: &LPckg,
        _pre_calc: &LPreCalc,
    ) {
        // Comment
        result.push_str(format!("    /// SUPER FIELD : {comment}\n", comment = class).as_str());
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

    /// Format "Simple property" from __get_all_simple_field__, to write field part
    fn format_field_simple_property(
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
                field_name = content.get_field_name(),
                field_type = content.get_field_type(&pre_calc),
            )
            .as_str(),
        );
    }

    /// Format "Complex property" from __get_all_complex_field__, to write field part
    fn format_field_complex_property(
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

    fn format_relation_super_from_one(
        class_model_name: &String,
        super_name: &String,
        field_name: &String,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        let key = super_name;
        if pre_calc.owned_member_type_list.contains_key(key) {
            let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
            let table_name = &matched_named.table_name;
            let model_name = &matched_named.model_name;
            let comment = format!("SUPER : ONE {} need ONE {}", class_model_name, model_name,);
            result.push_str(
                format!(
                    include_str!("../template/entity_sub_super_relation.tmpl"),
                    table_name = table_name,
                    model_name = model_name,
                    comment = comment,
                    foreign_field = field_name,
                )
                .as_str(),
            );
        }
    }

    fn format_relation_super_to_one(
        class_model_name: &String,
        super_name: &String,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        let key = super_name;
        if pre_calc.owned_member_type_list.contains_key(key) {
            let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
            let table_name = &matched_named.table_name;
            let model_name = &matched_named.model_name;
            let comment = format!("SUPER : ONE {} need ONE {}", model_name, class_model_name,);
            result.push_str(
                format!(
                    include_str!("../template/entity_sub_relation_to_one.tmpl"),
                    table_name = table_name,
                    model_name = model_name,
                    comment = comment,
                )
                .as_str(),
            );
        }
    }

    /// Format "Super" from __get_all_direct_super__, to write related part
    fn format_related_direct_super(
        class_model_name: &String,
        super_name: &String,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        let key = super_name;
        if pre_calc.owned_member_type_list.contains_key(key) {
            let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
            let table_name = &matched_named.table_name;
            let model_name = &matched_named.model_name;
            let comment = format!("SUPER : ONE {} need ONE {}", class_model_name, model_name,);
            result.push_str(
                format!(
                    include_str!("../template/entity_sub_super_related.tmpl"),
                    table_name = table_name,
                    model_name = model_name,
                    comment = comment,
                )
                .as_str(),
            );
        }
    }

    /// Format inverse of "Super" from __get_all_reverse_super__, to write related part
    fn format_related_reverse_super(
        class_model_name: &String,
        super_name: &String,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        let key = super_name;
        if pre_calc.owned_member_type_list.contains_key(key) {
            let matched_named = pre_calc.owned_member_type_list.get(key).unwrap();
            let table_name = &matched_named.table_name;
            let model_name = &matched_named.model_name;
            let comment = format!("SUPER : ONE {} need ONE {}", model_name, class_model_name,);
            result.push_str(
                format!(
                    include_str!("../template/entity_sub_super_related.tmpl"),
                    table_name = table_name,
                    model_name = model_name,
                    comment = comment,
                )
                .as_str(),
            );
        }
    }

    /// Format inverse of "Super" from __get_all_reverse_super__, to write related part
    fn format_related_many_to_many(
        association_name: &String,
        actual_relation: &ElementRelation,
        other_relation: &ElementRelation,
        result: &mut String,
        _pckg: &LPckg,
        pre_calc: &LPreCalc,
    ) {
        let association_named = pre_calc.owned_member_type_list.get(association_name);
        let association_table_name = if association_named.is_some() {
            &association_named.unwrap().table_name
        } else {
            &String::new()
        };
        let comment = format!(
            "ManyToMany : with {} using {}",
            other_relation.element_type, association_name,
        );
        result.push_str(
            format!(
                include_str!("../template/entity_sub_related_many_to_many.tmpl"),
                association_table_name = association_table_name,
                other_model = other_relation.element_type,
                actual_model = actual_relation.element_type,
                comment = comment,
            )
            .as_str(),
        );
    }

    // Return content for "help_doc" in "entity_main_class"
    fn get_help(&self, pckg: &LPckg, pre_calc: &LPreCalc) -> String {
        let mut result = String::new();

        // Add a head
        result.push_str(
            format!(
                "# Help document for \"{}\" ({})\n\n",
                self.get_model_name(),
                self.get_full_name(pckg)
            )
            .as_str(),
        );

        // Common
        result.push_str("## Common fields :\n");
        result.push_str("* __id__ (sea_orm only)\n");
        result.push_str("  * type : __i64__\n");
        result.push_str("\n");

        // Attribute : SIMPLE
        let iter_properties = self.get_all_simple_field();
        if iter_properties.len() > 0 {
            result.push_str("## Simple fields :\n");
        }
        for property in iter_properties {
            // Property head
            result.push_str(
                format!(
                    "* __{}__ (xmi_id : \"{}\")\n",
                    property.get_field_name(),
                    property.xmi_id
                )
                .as_str(),
            );

            // Property content
            result.push_str(
                format!("  * type : __{}__\n", property.get_field_type(pre_calc)).as_str(),
            );
            if property.default.is_some() {
                result.push_str(
                    format!("  * default : \"{}\"\n", property.default.as_ref().unwrap()).as_str(),
                );
            };
        }
        result.push_str("\n");

        // Attribute : Complex (direct One To One)
        let iter_direct_one_to_one = self.get_all_direct_one_to_one(pre_calc);
        if iter_direct_one_to_one.len() > 0 {
            result.push_str("## Direct One To One :\n");
        }
        for (association_name, association) in iter_direct_one_to_one {
            // Property head
            result.push_str(
                format!(
                    "* __{}__ (__{}Model__) from {}\n",
                    association.relation_2.element_type,
                    association.relation_2.element_type,
                    association_name
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * one-to-one link : ({}-{}) __{}__ need ({}-{}) __{}__)\n",
                    association.relation_2.lower,
                    association.relation_2.upper,
                    association.relation_1.element_type,
                    association.relation_1.lower,
                    association.relation_1.upper,
                    association.relation_2.element_type
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * callable using find_also_related(__{}Model__) from __{}__\n",
                    association.relation_2.element_type,
                    self.get_model_name()
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * saved in __{}__ field as foreing key\n",
                    association.relation_2.property_name.to_case(Case::Snake)
                )
                .as_str(),
            );
        }
        result.push_str("\n");

        // Attribute : Relation (direct One To Many)
        let iter_direct_one_to_many = self.get_all_direct_one_to_many(pre_calc);
        if iter_direct_one_to_many.len() > 0 {
            result.push_str("## Relation : One To Many :\n");
        }
        for (association_name, association) in iter_direct_one_to_many {
            // Property head
            result.push_str(
                format!(
                    "* __{}__ (__{}Model__) from {}\n",
                    association.relation_2.element_type,
                    association.relation_2.element_type,
                    association_name
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * one-to-many link : ({}-{}) __{}__ need ({}-{}) __{}__)\n",
                    association.relation_2.lower,
                    association.relation_2.upper,
                    association.relation_1.element_type,
                    association.relation_1.lower,
                    association.relation_1.upper,
                    association.relation_2.element_type
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * callable using find_with_related(__{}Model__) from __{}__\n",
                    association.relation_2.element_type,
                    self.get_model_name()
                )
                .as_str(),
            );
            if association.relation_1.from == RelationSource::FromClass {
                result.push_str(
                    format!(
                        "  * named {} in BPMN\n",
                        association.relation_2.property_name.to_case(Case::Snake)
                    )
                    .as_str(),
                );
            }
        }
        result.push_str("\n");

        // Attribute : Super (direct)
        let iter_direct_super = self.get_all_direct_super();
        if iter_direct_super.len() > 0 {
            result.push_str("## Direct Super :\n");
        }
        for direct_super in iter_direct_super {
            let field_name = &direct_super
                .to_case(Case::Snake)
                .prefix("super_")
                .replace("\n", "");
            // Property head
            result
                .push_str(format!("* __{}__ (__{}Model__)\n", direct_super, direct_super).as_str());
            result.push_str(
                format!(
                    "  * one-to-one link : one __{}__ need one __{}__)\n",
                    self.get_model_name(),
                    direct_super
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * callable using find_also_related(__{}Model__) from __{}__\n",
                    direct_super,
                    self.get_model_name()
                )
                .as_str(),
            );
            result.push_str(
                format!("  * saved in __{}__ field as foreing key\n", field_name).as_str(),
            );
        }

        // Attribute : Complex (Reverse One To One)
        let iter_reverse_one_to_one = self.get_all_reverse_one_to_one(pre_calc);
        if iter_reverse_one_to_one.len() > 0 {
            result.push_str("## Reverse One To One :\n");
        }
        for (association_name, association) in iter_reverse_one_to_one {
            // Property head
            result.push_str(
                format!(
                    "* __{}__ (__{}Model__) from {}\n",
                    association.relation_1.element_type,
                    association.relation_1.element_type,
                    association_name
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * one-to-one link : ({}-{}) __{}__ need ({}-{}) __{}__)\n",
                    association.relation_2.lower,
                    association.relation_2.upper,
                    association.relation_1.element_type,
                    association.relation_1.lower,
                    association.relation_1.upper,
                    association.relation_2.element_type
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * callable using find_also_related(__{}Model__) from __{}__\n",
                    self.get_model_name(),
                    association.relation_1.element_type
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * saved in __{}__ field as foreing key\n",
                    association.relation_2.property_name.to_case(Case::Snake)
                )
                .as_str(),
            );
        }
        result.push_str("\n");

        // Attribute : Super (reverse)
        let iter_reverse_super = self.get_all_reverse_super(pre_calc);
        if iter_reverse_super.len() > 0 {
            result.push_str("## Reverse Super :\n");
        }
        for reverse_super in iter_reverse_super {
            let field_name = &self
                .get_model_name()
                .to_case(Case::Snake)
                .prefix("super_")
                .replace("\n", "");
            // Property head
            result.push_str(
                format!("* __{}__ (__{}Model__)\n", reverse_super, reverse_super).as_str(),
            );
            result.push_str(
                format!(
                    "  * one-to-one link (reverse) : one __{}__ need one __{}__)\n",
                    reverse_super,
                    self.get_model_name()
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * callable using find_also_related(__{}Model__) from __{}__\n",
                    self.get_model_name(),
                    reverse_super
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "  * saved in __{}__ field as foreing key in __{}Model__\n",
                    field_name, reverse_super
                )
                .as_str(),
            );
        }
        result.push_str("\n");

        result
    }

    // /// Format of "Super" from __get_all_direct_super__, to write related part
    // fn format_related_reverse_super(
    //     class_model_name: &String,
    //     super_name: &String,
    //     result: &mut String,
    //     _pckg: &LPckg,
    //     pre_calc: &LPreCalc,
    // ) {
    // }
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
                // Simple field, i.e. other Enumeration
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
                        "i32"
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
        result.push_str(if self.lower == 0 { ">" } else { "" });

        result
    }

    fn get_field_name(&self) -> String {
        if &self.name.to_case(Case::Snake) == &String::from("id") {
            String::from("bpmn_id")
        } else {
            self.name.to_case(Case::Snake)
        }
    }
}
