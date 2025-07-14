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
#![doc = include_str!("../doc/writing_manager.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_cmof_structure::*;
use crate::loader_dependencies_explorer::*;

use infinitable::Infinitable;
// Dependencies section
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

// ####################################################################################################
//
// ###################################### NamingStruct ################################################
//
// ####################################################################################################

/// Naming method for providing struct name
pub trait NamingStruct {
    /// --> DC.cmof#Font
    fn get_technical_name(&self, package: &LoadingPackage) -> String;
    /// --> dc_font
    fn get_table_name(&self, package: &LoadingPackage) -> String;
    /// --> Font
    fn get_model_name(&self) -> String;
    /// --> dc_datatype_font
    fn get_full_name(&self, package: &LoadingPackage) -> String;
}

impl NamingStruct for EnumOwnedMember {
    fn get_technical_name(&self, package: &LoadingPackage) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.get_technical_name(package),
            EnumOwnedMember::Class(content) => content.get_technical_name(package),
            EnumOwnedMember::DataType(content) => content.get_technical_name(package),
            EnumOwnedMember::Enumeration(content) => content.get_technical_name(package),
            EnumOwnedMember::PrimitiveType(content) => content.get_technical_name(package),
        }
    }
    fn get_table_name(&self, package: &LoadingPackage) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.get_table_name(package),
            EnumOwnedMember::Class(content) => content.get_table_name(package),
            EnumOwnedMember::DataType(content) => content.get_table_name(package),
            EnumOwnedMember::Enumeration(content) => content.get_table_name(package),
            EnumOwnedMember::PrimitiveType(content) => content.get_table_name(package),
        }
    }
    fn get_model_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.get_model_name(),
            EnumOwnedMember::Class(content) => content.get_model_name(),
            EnumOwnedMember::DataType(content) => content.get_model_name(),
            EnumOwnedMember::Enumeration(content) => content.get_model_name(),
            EnumOwnedMember::PrimitiveType(content) => content.get_model_name(),
        }
    }
    fn get_full_name(&self, package: &LoadingPackage) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.get_full_name(package),
            EnumOwnedMember::Class(content) => content.get_full_name(package),
            EnumOwnedMember::DataType(content) => content.get_full_name(package),
            EnumOwnedMember::Enumeration(content) => content.get_full_name(package),
            EnumOwnedMember::PrimitiveType(content) => content.get_full_name(package),
        }
    }
}

impl NamingStruct for CMOFAssociation {
    fn get_technical_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name);
        result.push_str(".cmof#");
        result.push_str(self.name.as_str());
        result
    }
    fn get_table_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
    fn get_model_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
    fn get_full_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_association_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
}

impl NamingStruct for CMOFClass {
    fn get_technical_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name);
        result.push_str(".cmof#");
        result.push_str(self.name.as_str());
        result
    }
    fn get_table_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
    fn get_model_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
    fn get_full_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_class_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
}

impl NamingStruct for CMOFDataType {
    fn get_technical_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name);
        result.push_str(".cmof#");
        result.push_str(self.name.as_str());
        result
    }
    fn get_table_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
    fn get_model_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
    fn get_full_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_datatype_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
}

impl NamingStruct for CMOFEnumeration {
    fn get_technical_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name);
        result.push_str(".cmof#");
        result.push_str(self.name.as_str());
        result
    }
    fn get_table_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
    fn get_model_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
    fn get_full_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_enumeration_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
}

impl NamingStruct for CMOFPrimitiveType {
    fn get_technical_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name);
        result.push_str(".cmof#");
        result.push_str(self.name.as_str());
        result
    }
    fn get_table_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
    fn get_model_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
    fn get_full_name(&self, package: &LoadingPackage) -> String {
        let mut result = String::from("");
        result.push_str(&package.get_json().name.to_case(Case::Snake).as_str());
        result.push_str("_primitive_");
        result.push_str(self.name.as_str().to_case(Case::Snake).as_str());
        result
    }
}

// ####################################################################################################
//
// ###################################### WrittingPath ################################################
//
// ####################################################################################################

/// Trait providing full homogenous path to [`LoadingTracker`]
pub trait WrittingPath {
    /// Get lib.rs file for the LoadingTracker
    ///
    /// Example --> ${output_folder}/src/lib.rs
    fn get_project_lib_file(&self) -> (PathBuf, File);

    /// Get ${package}.rs file for a object of a package
    ///
    /// Example for font object of dc package --> ${output_folder}/src/dc/font.rs
    fn get_object_file(
        &self,
        package: &LoadingPackage,
        object: &EnumOwnedMember,
    ) -> (PathBuf, File);
}

impl WrittingPath for LoadingTracker {
    fn get_project_lib_file(&self) -> (PathBuf, File) {
        // Calculate path
        let mut file_name = self.get_output_folder();
        file_name.push("lib.rs");
        // Create file
        (file_name.clone(), file_name.write_new_file())
    }

    fn get_object_file(
        &self,
        package: &LoadingPackage,
        object: &EnumOwnedMember,
    ) -> (PathBuf, File) {
        // Calculate path
        let mut file_name = self.get_output_folder();
        file_name.push(object.get_table_name(package) + ".rs");
        // Create file
        (file_name.clone(), file_name.write_new_file())
    }
}

// ####################################################################################################
//
// ##################################### LoadingTracker ###############################################
//
// ####################################################################################################

#[derive(Debug, Deserialize)]
struct SimpleValue {
    pub key: String,
    pub value: String,
}

impl LoadingTracker {
    /// Build all pre calculing information needed for writting
    pub fn writing_preparation(&mut self) {
        // owned_member_type_list
        let mut result: HashMap<String, Named> = HashMap::new();
        for (_, package) in self.get_package_in_order() {
            for owned_member in package.get_sorted_owned_member() {
                let key = match owned_member {
                    EnumOwnedMember::Association(content) => content.name.clone(),
                    _ => owned_member.get_model_name().clone(),
                };
                let value = Named {
                    package_name: package.get_lowercase_name(),
                    technical_name: owned_member.get_technical_name(package),
                    table_name: owned_member.get_table_name(package),
                    model_name: owned_member.get_model_name(),
                    full_name: owned_member.get_full_name(package),
                };
                result.insert(key, value);
            }
        }
        self.pre_calculation.owned_member_type_list = result.clone();
        // debug!(
        //     "Writing_preparation : owned_member_type_list {:#?}",
        //     self.pre_calculation.owned_member_type_list
        // );

        // enumeration_default_value
        let reader_path = Path::new("metamodel_file_extension/enumeration_default_value.json");
        let reader = reader_path.get_file_content();
        let values: Vec<SimpleValue> = serde_json::from_str(&reader).unwrap();
        for import_simple_value in values {
            self.pre_calculation
                .enumeration_default_value
                .insert(import_simple_value.key, import_simple_value.value);
        }
        // debug!(
        //     "Writing_preparation : enumeration_default_value {:#?}",
        //     self.pre_calculation.enumeration_default_value
        // );

        // primitive_type_conversion
        let reader_path = Path::new("metamodel_file_extension/primitive_type_conversion.json");
        let reader = reader_path.get_file_content();
        let values: Vec<SimpleValue> = serde_json::from_str(&reader).unwrap();
        for import_simple_value in values {
            self.pre_calculation
                .primitive_type_conversion
                .insert(import_simple_value.key, import_simple_value.value);
        }
        // debug!(
        //     "Writing_preparation : primitive_type_conversion {:#?}",
        //     self.pre_calculation.primitive_type_conversion
        // );

        // association_relation
        let mut result: HashMap<String, Vec<ElementRelation>> = HashMap::new();
        for (_, package) in self.get_package_in_order() {
            for owned_member in package.get_sorted_owned_member() {
                match owned_member {
                    EnumOwnedMember::Association(content) => {
                        let key = content.name.clone();
                        for owned_end in &content.owned_end {
                            match owned_end {
                                EnumOwnedEnd::Property(property) => {
                                    let value = ElementRelation {
                                        element_type: property.get_type(),
                                        property_name: property.name.clone(),
                                        lower: property.lower,
                                        upper: property.upper,
                                        from: RelationSource::FromAssociation,
                                    };
                                    if result.contains_key(&key) {
                                        let result_vec = result.get_mut(&key).unwrap();
                                        result_vec.push(value);
                                    } else {
                                        result.insert(key.clone(), Vec::from([value]));
                                    };
                                }
                            }
                        }
                    }
                    EnumOwnedMember::Class(content) => {
                        for owned_attribute in &content.owned_attribute {
                            match owned_attribute {
                                EnumOwnedAttribute::Property(property) => {
                                    if property.association.is_some() {
                                        let key = property.association.clone().unwrap();

                                        let value = ElementRelation {
                                            element_type: property.get_type(),
                                            property_name: property.name.clone(),
                                            lower: property.lower,
                                            upper: property.upper,
                                            from: RelationSource::FromClass,
                                        };
                                        if result.contains_key(&key) {
                                            let result_vec = result.get_mut(&key).unwrap();
                                            result_vec.push(value);
                                        } else {
                                            result.insert(key.clone(), Vec::from([value]));
                                        };
                                    };
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        for (relation_name, association) in result {
            if association.len() != 2 {
                error!(
                    "Association multiplicity error : need to be 2, not {num} : {name}",
                    num = association.len(),
                    name = relation_name,
                );
            } else {
                let key = relation_name;
                let relation_1 = if association[0].upper < association[1].upper {
                    association[1].clone()
                } else {
                    association[0].clone()
                };
                let relation_2 = if association[0].upper < association[1].upper {
                    association[0].clone()
                } else {
                    association[1].clone()
                };
                let ponteration_type = if relation_2.upper > Infinitable::Finite(1) {
                    RelationPonderationType::ManyToMany
                } else if relation_1.upper > Infinitable::Finite(1) {
                    RelationPonderationType::OneToMany
                } else {
                    RelationPonderationType::OneToOne
                };
                let is_self_referencing =
                    association[0].element_type == association[1].element_type;
                let value = AssociationRelation {
                    relation_1: relation_1,
                    relation_2: relation_2,
                    ponteration_type: ponteration_type,
                    is_self_referencing: is_self_referencing,
                };
                self.pre_calculation.association_relation.insert(key, value);
            }
        }
        debug!(
            "Writing_preparation : association_relation {:#?}",
            self.pre_calculation.association_relation
        );

        // association_relation_by_class
        let mut result: HashMap<String, Vec<(String, RankRelation)>> = HashMap::new();
        for (name, association) in &self.pre_calculation.association_relation {
            // Relation 1
            let member = &association.relation_1;
            let key = member.element_type.clone();
            let value = (name.clone(), RankRelation::IsOne);

            if result.contains_key(&key) {
                let result_vec = result.get_mut(&key).unwrap();
                result_vec.push(value);
            } else {
                result.insert(key.clone(), Vec::from([value]));
            };
            // Relation 2
            let member = &association.relation_2;
            let key = member.element_type.clone();
            let value = (name.clone(), RankRelation::IsSecond);

            if result.contains_key(&key) {
                let result_vec = result.get_mut(&key).unwrap();
                result_vec.push(value);
            } else {
                result.insert(key.clone(), Vec::from([value]));
            };
        }
        self.pre_calculation.association_relation_by_class = result.clone();
        // debug!(
        //     "Writing_preparation : association_relation_by_class {:#?}",
        //     self.pre_calculation.association_relation_by_class
        // );

        // reverse_super_link
        let mut result: HashMap<String, Vec<String>> = HashMap::new();
        for (_, package) in self.get_package_in_order() {
            for owned_member in package.get_sorted_owned_member() {
                match owned_member {
                    EnumOwnedMember::Class(content) => {
                        // As default, empty
                        let mut list_of_super: Vec<String> = content.super_class.clone();

                        // // For super class link
                        for link in content.super_class_link.clone() {
                            match link {
                                EnumSuperClass::Class(content) => {
                                    let class = content.href.clone();
                                    let class = match class.find(".cmof#") {
                                        Some(split_index) => {
                                            class[split_index..].replace(".cmof#", "").to_string()
                                        }
                                        None => class,
                                    };
                                    list_of_super.push(class);
                                }
                            }
                        }

                        // for all element in result
                        for super_field in list_of_super {
                            let key = super_field.clone();
                            let value = content.get_model_name();

                            if result.contains_key(&key) {
                                let result_vec = result.get_mut(&key).unwrap();
                                result_vec.push(value);
                            } else {
                                result.insert(key.clone(), Vec::from([value]));
                            };
                        }
                    }
                    _ => {}
                }
            }
        }
        self.pre_calculation.reverse_super_link = result.clone();
        // debug!(
        //     "Writing_preparation : reverse_super_link {:#?}",
        //     self.pre_calculation.reverse_super_link
        // );
    }
}

impl CMOFProperty {
    fn get_type(&self) -> String {
        let mut result = String::new();

        // For field simple
        let content = if self.simple_type.is_some() {
            self.simple_type.as_ref().unwrap()
        } else {
            match self.complex_type.as_ref().unwrap() {
                EnumType::PrimitiveTypeLink(link) => {
                    // Foreign field
                    let key = link.href.clone();
                    let key = match key.find(".cmof#") {
                        Some(split_index) => key[split_index..].replace(".cmof#", ""),
                        None => key,
                    };
                    &key.clone()
                }
                EnumType::ClassLink(link) => {
                    // Foreign field
                    let key = link.href.clone();
                    let key = match key.find("http://schema.omg.org/spec/MOF/2.0/cmof.xml#") {
                        Some(split_index) => key[split_index..]
                            .replace(
                                "http://schema.omg.org/spec/MOF/2.0/cmof.xml#",
                                "Extensibilty.cmof#",
                            )
                            .to_string(),
                        None => key,
                    };
                    let key = match key.find(".cmof#") {
                        Some(split_index) => key[split_index..].replace(".cmof#", "").to_string(),
                        None => key,
                    };
                    &key.clone()
                }
                EnumType::DataTypeLink(link) => {
                    // Foreign field
                    let key = link.href.clone();
                    let key = match key.find(".cmof#") {
                        Some(split_index) => key[split_index..].replace(".cmof#", "").to_string(),
                        None => key,
                    };
                    &key.clone()
                }
            }
        };
        result.push_str(content);

        result
    }
}

// ####################################################################################################
//
// ################################### Writting Organiser #############################################
//
// ####################################################################################################

/// Trait for writting __lib.rs__ file from sub-element of [`LoadingPackage`]
pub trait WritingLibFile: Debug {
    /// Writting __lib.rs__ file from sub-element of [`LoadingPackage`]
    fn wrt_lib_file_level(
        &self,
        wrt: &mut File,
        pckg: &LoadingPackage,
        pre_calc: &LoadingPreCalculation,
    );
}

/// Trait for writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModObject: Debug {
    /// Writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
    fn wrt_entity_fields(
        &self,
        wrt: &mut File,
        pckg: &LoadingPackage,
        pre_calc: &LoadingPreCalculation,
    );
}

/// Trait for writting __${owned_member}.rs__ struct validation from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModValidation: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : macro adding struct validation
    fn wrt_sub_validation(&self, writer: &mut File);
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : additionnal validation function for struct validation
    fn wrt_main_validation(&self, writer: &mut File);
}
