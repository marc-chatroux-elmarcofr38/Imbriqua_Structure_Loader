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
#![doc = include_str!("../../doc/writing_manager.md")]

// Package section
use crate::cmof_loader::*;
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

// Dependencies section
use std::fmt::Debug;

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
    fn get_object_file(&self, object: &EnumOwnedMember) -> (PathBuf, File);
}

impl WrittingPath for LoadingTracker {
    fn get_project_lib_file(&self) -> (PathBuf, File) {
        // Calculate path
        let mut file_name = self.get_output_folder();
        file_name.push("lib.rs");
        // Create file
        (file_name.clone(), file_name.write_new_file().unwrap())
    }

    fn get_object_file(&self, object: &EnumOwnedMember) -> (PathBuf, File) {
        // Calculate path
        let mut file_name = self.get_output_folder();
        file_name.push(object.get_table_name() + ".rs");
        // Create file
        (file_name.clone(), file_name.write_new_file().unwrap())
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
    fn wrt_lib_file_level(&self, wrt: &mut File) -> Result<(), anyhow::Error>;
}

/// Trait for writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModObject: Debug {
    /// Writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element of [`LoadingPackage`]
    fn wrt_entity_fields(&self, wrt: &mut File) -> Result<(), anyhow::Error>;
}

/// Trait for writting __${owned_member}.rs__ struct validation from [`EnumOwnedMember`] element of [`LoadingPackage`]
pub trait WritingModValidation: Debug {
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : macro adding struct validation
    fn wrt_sub_validation(&self, writer: &mut File) -> Result<(), anyhow::Error>;
    /// Implement writing of target struct instance as Rust struct format
    /// Writing section : additionnal validation function for struct validation
    fn wrt_main_validation(&self, writer: &mut File) -> Result<(), anyhow::Error>;
}
