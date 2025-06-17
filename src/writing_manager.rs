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

// Dependencies section
use serde::Deserialize;
use std::fmt::Debug;

// ####################################################################################################
//
// ######################################## NamingLink ################################################
//
// ####################################################################################################

// /// Naming method for struct in package
// pub trait NamingLink {
//     /// Naming method for struct ([`EnumOwnedMember`]) in package ([`LoadingPackage`]), as hierarchical position
//     ///
//     /// Usecase :
//     /// - Usable for calling Resultant Struct from other package
//     ///
//     ///
//     /// Example : with package dc and datatype_font
//     ///   ---> "dc::Font"
//     fn get_cmof_name(&self, _package: &LoadingPackage) -> String;
// }

// impl NamingLink for EnumOwnedMember {
//     fn get_cmof_name(&self, _package: &LoadingPackage) -> String {
//         let mut result = _package.get_lowercase_name();
//         result.push_str("::");
//         result.push_str(self.get_model_name().as_str());
//         result
//     }
// }

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
#[serde(deny_unknown_fields)]
struct SimpleValue {
    pub key: String,
    pub value: String,
    comment: String,
}

impl LoadingTracker {
    /// Build all pre calculing information needed for writting
    pub fn writing_preparation(&mut self) {
        // owned_member_type_list
        for (_, package) in self.clone().get_package_in_order() {
            for owned_member in package.get_sorted_iter() {
                let mut real_key = package.get_lowercase_name();
                real_key.push('_');
                real_key.push_str(owned_member.get_model_name().as_str());
                let v = match owned_member {
                    EnumOwnedMember::Association(_) => ClassType::Association,
                    EnumOwnedMember::Class(_) => ClassType::Class,
                    EnumOwnedMember::DataType(_) => ClassType::DataType,
                    EnumOwnedMember::Enumeration(_) => ClassType::Enumeration,
                    EnumOwnedMember::PrimitiveType(_) => ClassType::PrimitiveType,
                };
                self.pre_calculation
                    .owned_member_type_list
                    .push((real_key, v));
            }
        }
        self.pre_calculation
            .owned_member_type_list
            .sort_by(|(a, _), (b, _)| a.cmp(&b));
        debug!(
            "Writing_preparation : owned_member_type_list {:#?}",
            self.pre_calculation.owned_member_type_list
        );

        // enumeration_default_value
        let reader_path = Path::new("metamodel_file_extension/enumeration_default_value.json");
        let reader = reader_path.get_file_content();
        let values: Vec<SimpleValue> = serde_json::from_str(&reader).unwrap();
        for import_simple_value in values {
            self.pre_calculation
                .enumeration_default_value
                .insert(import_simple_value.key, import_simple_value.value);
        }
        debug!(
            "Writing_preparation : enumeration_default_value {:#?}",
            self.pre_calculation.enumeration_default_value
        );

        // primitive_type_conversion
        let reader_path = Path::new("metamodel_file_extension/primitive_type_conversion.json");
        let reader = reader_path.get_file_content();
        let values: Vec<SimpleValue> = serde_json::from_str(&reader).unwrap();
        for import_simple_value in values {
            self.pre_calculation
                .primitive_type_conversion
                .insert(import_simple_value.key, import_simple_value.value);
        }
        debug!(
            "Writing_preparation : primitive_type_conversion {:#?}",
            self.pre_calculation.primitive_type_conversion
        );
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
        writer: &mut File,
        package: &LoadingPackage,
        pre_calculation: &LoadingPreCalculation,
    );
}

/// Trait for dispatch run for writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModObjectCaller: Debug {
    /// Dispatch run for writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
    fn wrt_entity_fields_caller(
        &self,
        writer: &mut File,
        package: &LoadingPackage,
        pre_calculation: &LoadingPreCalculation,
    );
}

/// Trait for writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModObject: Debug {
    /// Writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
    fn wrt_entity_fields(
        &self,
        writer: &mut File,
        package: &LoadingPackage,
        pre_calculation: &LoadingPreCalculation,
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
