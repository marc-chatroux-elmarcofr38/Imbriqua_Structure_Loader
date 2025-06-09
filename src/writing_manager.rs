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
use std::fmt::Debug;

// ####################################################################################################
//
// ######################################## NamingLink ################################################
//
// ####################################################################################################

/// Naming method for struct in package
pub trait NamingLink {
    /// Naming method for struct ([`EnumOwnedMember`]) in package ([`LoadingPackage`]), as hierarchical position
    ///
    /// Usecase :
    /// - Usable for calling Resultant Struct from other package
    ///
    ///
    /// Example : with package dc and datatype_font
    ///   ---> "dc::Font"
    fn get_cmof_name(&self, _package: &LoadingPackage) -> String;
}

impl NamingLink for EnumOwnedMember {
    fn get_cmof_name(&self, _package: &LoadingPackage) -> String {
        let mut result = _package.get_lowercase_name();
        result.push_str("::");
        result.push_str(self.get_struct_name().as_str());
        result
    }
}

// ####################################################################################################
//
// ####################################### NamingPath #################################################
//
// ####################################################################################################

/// Naming method for providing path
pub trait NamingPath {
    /// Naming method for providing path to [`LoadingPackage`], [`ImportedPackage`] and [`EnumOwnedMember`]
    ///
    /// Usecase :
    /// - Usable for get the futur path of a [`LoadingPackage`]
    /// - Usable for get the futur path of a [`ImportedPackage`]
    ///     - Refer to a [`LoadingPackage`]
    /// - Usable for get the futur path of a [`EnumOwnedMember`]
    ///
    ///
    /// Example : with package dc and datatype_font
    ///   ---> "dc::Font"
    fn get_path_name(&self) -> String;
}

impl NamingPath for LoadingPackage {
    fn get_path_name(&self) -> String {
        self.get_lowercase_name().to_case(Case::Snake)
    }
}

impl NamingPath for ImportedPackage {
    fn get_path_name(&self) -> String {
        let content = self.href.clone();
        let content = content.replace(".cmof#_0", "");
        let mut result = String::from("");
        result.push_str(content.to_case(Case::Snake).as_str());
        result
    }
}

impl NamingPath for EnumOwnedMember {
    fn get_path_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.get_path_name(),
            EnumOwnedMember::Class(content) => content.get_path_name(),
            EnumOwnedMember::DataType(content) => content.get_path_name(),
            EnumOwnedMember::Enumeration(content) => content.get_path_name(),
            EnumOwnedMember::PrimitiveType(content) => content.get_path_name(),
        }
    }
}

impl NamingPath for CMOFAssociation {
    fn get_path_name(&self) -> String {
        self.name
            .to_case(Case::Snake)
            .prefix("link_")
            .replace("\n", "")
    }
}

impl NamingPath for CMOFClass {
    fn get_path_name(&self) -> String {
        self.name
            .to_case(Case::Snake)
            .prefix("class_")
            .replace("\n", "")
    }
}

impl NamingPath for CMOFDataType {
    fn get_path_name(&self) -> String {
        self.name
            .to_case(Case::Snake)
            .prefix("datatype_")
            .replace("\n", "")
    }
}

impl NamingPath for CMOFEnumeration {
    fn get_path_name(&self) -> String {
        self.name
            .to_case(Case::Snake)
            .prefix("enum_")
            .replace("\n", "")
    }
}

impl NamingPath for CMOFPrimitiveType {
    fn get_path_name(&self) -> String {
        self.name
            .to_case(Case::Snake)
            .prefix("primitivetype_")
            .replace("\n", "")
    }
}

// ####################################################################################################
//
// ###################################### NamingStruct ################################################
//
// ####################################################################################################

/// Naming method for providing struct name
pub trait NamingStruct {
    /// Naming method for providing strct name to [`EnumOwnedMember`]
    ///
    /// Usecase :
    /// - Usable for get the strct name of a [`EnumOwnedMember`]
    ///
    ///
    /// Example : with package dc and datatype_font
    ///   ---> datatpe_font
    fn get_struct_name(&self) -> String;
}

impl NamingStruct for EnumOwnedMember {
    fn get_struct_name(&self) -> String {
        match self {
            EnumOwnedMember::Association(content) => content.get_struct_name(),
            EnumOwnedMember::Class(content) => content.get_struct_name(),
            EnumOwnedMember::DataType(content) => content.get_struct_name(),
            EnumOwnedMember::Enumeration(content) => content.get_struct_name(),
            EnumOwnedMember::PrimitiveType(content) => content.get_struct_name(),
        }
    }
}

impl NamingStruct for CMOFAssociation {
    fn get_struct_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
}

impl NamingStruct for CMOFClass {
    fn get_struct_name(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.as_str());
        result
    }
}

impl NamingStruct for CMOFDataType {
    fn get_struct_name(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}

impl NamingStruct for CMOFEnumeration {
    fn get_struct_name(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}

impl NamingStruct for CMOFPrimitiveType {
    fn get_struct_name(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.name.to_case(Case::UpperCamel).as_str());
        result
    }
}

// ####################################################################################################
//
// ###################################### WrittingPath ################################################
//
// ####################################################################################################

/// Naming method for creating full path to Package and EnumOwnedMember, including __outpt_result_manager__ output folder by using LoadingTracker
pub trait WrittingPath {
    /// Get lib.rs file for the LoadingTracker
    ///
    /// Example --> $output_folder$/src/lib.rs
    fn get_project_lib_file(&self) -> (PathBuf, File);

    /// Get output folder for the package
    ///
    /// Example for dc package --> $output_folder$/src/dc
    fn get_package_folder(&self, package: &LoadingPackage) -> PathBuf;

    /// Get mod.rs file for the package
    ///
    /// Example for dc package --> $output_folder$/src/dc/mod.rs
    fn get_package_mod_file(&self, package: &LoadingPackage) -> (PathBuf, File);

    /// Get $.rs file for a object of a package
    ///
    /// Example for font object of dc package --> $output_folder$/src/dc/font.rs
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
    fn get_package_folder(&self, package: &LoadingPackage) -> PathBuf {
        // Calculate path
        let mut folder_name = self.get_output_folder();
        folder_name.push(package.get_path_name());
        // Create empty folder
        folder_name.create_folder();
        folder_name
    }

    fn get_package_mod_file(&self, package: &LoadingPackage) -> (PathBuf, File) {
        // Calculate path
        let mut file_name = self.get_output_folder();
        file_name.push(package.get_path_name());
        file_name.push("mod.rs");
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
        file_name.push(package.get_path_name());
        file_name.push(object.get_path_name() + ".rs");
        // Create file
        (file_name.clone(), file_name.write_new_file())
    }
}

// ####################################################################################################
//
// ##################################### LoadingTracker ###############################################
//
// ####################################################################################################

impl LoadingTracker {
    /// Build all pre calculing information
    pub fn writing_preparation(&mut self) {
        for (_, package) in self.clone().get_package_in_order() {
            for owned_member in package.get_json().get_sorted_iter() {
                let mut real_key = package.get_lowercase_name();
                real_key.push('_');
                real_key.push_str(owned_member.get_path_name().as_str());
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

        // Alone classes
        // for (_, package) in self.clone().get_package_in_order() {
        //     for owned_member in package.get_json().get_sorted_iter() {
        //         match owned_member {
        //             EnumOwnedMember::Class(content) => {
        //                 let mut is_alone = true;
        //                 for owned_attribute in content.owned_attribute {
        //                     match owned_attribute {
        //                         EnumOwnedAttribute::Property(content_2) => {
        //                             if !is_simple_dpt(content_2.name.as_str()) {
        //                                 is_alone = false;
        //                             }
        //                         }
        //                     }
        //                 }
        //                 if is_alone {
        //                     self.pre_calculation
        //                         .class_classification
        //                         .insert(content.name, ClassClassification::Simple);
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        // debug!(
        //     "Simple Class {:?}",
        //     self.pre_calculation.class_classification
        // );
        // build_class_classification();
    }
}

/// Bullshit function : define if a type (represent as string.....) involve to set a structure using reference ("&")
pub fn is_simple_dpt(input: &str) -> bool {
    match input {
        "Boolean" => false,
        "Integer" => false,
        "Real" => false,
        "String" => false,
        "i8" => false,
        "u8" => false,
        "dc::Boolean" => false,
        "dc::Integer" => false,
        "dc::Real" => false,
        "dc::String" => false,
        _ => true,
    }
}

// ####################################################################################################
//
// ################################### Writting Organiser #############################################
//
// ####################################################################################################

/// Implement writing of target lib loading element as Rust
pub trait WritingLibFileHead: Debug {
    /// Implement writing of target lib loading element as Rust
    fn wrt_lib_file_level(&self, writer: &mut File);
}

/// Implement writing of target mod loading head element as Rust
pub trait WritingModFileHead: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : module file head (import part, "use", etc.)
    fn wrt_mod_file_head(&self, writer: &mut File, pre_calculation: &LoadingPreCalculation);
}

/// Implement writing of target mod loading head element as Rust
pub trait WritingModFileObjectSection: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : struct element (macro for struct and struct)
    fn wrt_mod_file_object_section(
        &self,
        writer: &mut File,
        pre_calculation: &LoadingPreCalculation,
    );
}

// /// Implement writing of target struct instance as Rust struct trait implementation
// pub trait WritingModTrait: Debug {
//     /// Implement writing of target struct instance as Rust struct trait implementation
//     fn wrt_trait_level(&self, writer: &mut File);
// }

/// Implement writing of target mod loading head element as Rust
pub trait WritingCallModObject: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : struct element (macro for struct and struct)
    fn wrt_call_mod_object(
        &self,
        writer: &mut File,
        pre_calculation: &LoadingPreCalculation,
        package: &LoadingPackage,
    );
}

/// Implement writing of target mod loading head element as Rust
pub trait WritingModObject: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : struct element (macro for struct and struct)
    fn wrt_mod_object(&self, writer: &mut File);
}

/// Implement writing of target struct validationfunction as Rust format
pub trait WritingModValidation: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : macro adding struct validation
    fn wrt_sub_validation(&self, writer: &mut File);
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : additionnal validation function for struct validation
    fn wrt_main_validation(&self, writer: &mut File);
}
