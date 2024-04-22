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

#![warn(missing_docs)]
#![doc = include_str!("../doc/module_dependencies_explorer.md")]

// Package section
use crate::module_file_manager::{FileManager, PathBuf, Path};
use crate::module_log::*;
use crate::module_file_env::*;

// Dependencies section
extern crate minidom;
use std::collections::HashMap;
use std::io::Write;
use std::fs::canonicalize;
use std::fmt;
use chrono::Local;
use log::{error, info, trace};
use minidom::Element;

#[derive(Clone, PartialEq, Debug)]
enum LoadingState {
    Empty,      // No Element
    Loaded,     // With Element
    _Finished,   // Element converted
}

#[derive(Clone, PartialEq, Debug)]
struct LoadingPackage {
    filename : String,
    id : String,
    object : Element,
    state : LoadingState,
}

impl LoadingPackage {
    pub fn get_lowercase_name(&self) -> String {
        let str_result = self.filename.as_str().to_ascii_lowercase();
        let str_result = str_result.replace(".", "_");
        let str_result = str_result.replace("#", "_");
        String::from("cmof_") + str_result.as_str() + self.id.as_str().to_ascii_lowercase().as_str()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LoadingTracker {
    pub file_env : FileEnv,
    loaded_package : HashMap<String, LoadingPackage>,
    importing_order : HashMap<String, usize>,
}

impl LoadingTracker {
    pub fn new(file_env : FileEnv) -> Self {

        // Create instance
        let result = LoadingTracker {
            file_env : file_env,
            loaded_package : HashMap::new(),
            importing_order : HashMap::new(),
        };

        // Return result
        result
    }

    pub fn import_dependencies_file(&mut self, main_file : &str, package_id : &str, parent_label : &str) {
        /*
            Load minidom element from a gived package, including dependencies
            Save element in loaded_package

            Input :
             - main_file (&str) : file to load in self.input_folder
             - package_id (&str) : package name wanted in main_file
             - parent_label (&str) : parent package label, used in logs command

            Error :
             - none
        */

        // Define hashmap key
        let mut label = String::from(main_file);
        label.push_str("#");
        label.push_str(package_id);

        // Check if the loading is necessary
        if self.is_package_already_loaded(label.clone()) && self.loaded_package.get_key_value(&label.clone()).unwrap().1.state == LoadingState::Empty {
            error!("ERROR_FILE07 - Unloaded dependencies : suspicious of circular dependencies ({a} importing {b})", a=label.clone(), b=parent_label);
            panic!("PANIC_FILE07 - Unloaded dependencies : suspicious of circular dependencies ({a} importing {b})", a=label.clone(), b=parent_label);
        }else if self.is_package_already_loaded(label.clone()) {
            trace!("Loading \"{}\" : NOPE : already loaded", label.clone());
            return
        } else {
            trace!("Loading \"{}\" : START", label.clone());
        }

        // Add empty element entry in loaded_package (prevent circular loading)
        self.add_empty_package(main_file, package_id, label.clone());

        // Generate file path
        let mut file_path = self.file_env.get_input_folder();
        file_path.push(main_file);

        // Load package element
        let package_element = file_path.get_file_content_as_element();
        // module_file_manager::get_package_from_path(file_path.as_str(), package_id);



        // Find "package_id" child
        for child in package_element.children() {
            if child.is("Package", "http://schema.omg.org/spec/MOF/2.0/cmof.xml") {
                if child.attr("xmi:id") == Some(package_id) {
                    let package_element = child.clone();
                    break
                }
            }
        };



        // Evaluate dependencies, and load it
        self.add_dependencies(package_element.clone(), label.clone());

        // Add package element in loaded_package
        self.add_package(package_element.clone(), main_file, package_id, label.clone());

        // End logs
        info!("Loading \"{}\" : Finished", label.clone());
    }

    fn add_empty_package(&mut self, file : &str, package : &str, label : String) {
        /*
            Save minimal LoadingPackage object in loaded_package
            Set "state" to empty to prevent circular dependencies loading (dependencies has loaded before changing "state")

            Input :
             - file (&str) : file to load in self.input_folder
             - package (&str) : package name wanted in main_file
             - label (String) : label of the package

            Error :
             - none
        */

        // Create minimal LoadingPackage
        let package_object = LoadingPackage {
            filename : String::from(file),
            id : String::from(package),
            object : Element::builder("", "").build(),
            state : LoadingState::Empty,
        };

        // Save object in hashmap attribute
        self.loaded_package.insert(label, package_object);
    }

    fn add_dependencies(&mut self, element : Element, label : String) {
        /*
            Get dependencies of a Element (UML XMI notation)
            And import dependencies

            Input :
             - element (Element) : Minidom ELement object to analyse
             - label (String) : package label

            Error :
             - none
        */

        for child in element.children() {
            if child.is("packageImport", "") {
                // Go to "importedPackage" child
                let imported_package = match child.get_child("importedPackage", "") {
                    Some(result_object) => {
                        result_object
                    },
                    None => {
                        error!("ERROR_FILE08 - packageImport element without importedPackage child : package = \"{}\"", label);
                        panic!("PANIC_FILE08 - packageImport element without importedPackage child : package = \"{}\"", label);
                    },
                };

                // Get "href" attribute
                let package_to_import = match imported_package.attr("href") {
                    Some(result_object) => {
                        result_object
                    },
                    None => {
                        error!("ERROR_FILE09 - importedPackage element without href attribute : package = \"{}\"", label);
                        panic!("PANIC_FILE09 - importedPackage element without href attribute : package = \"{}\"", label);
                    },
                };

                //
                match package_to_import.find('#') {
                    Some(split_index) => {
                        trace!("Loading \"{}\" : need to load \"{}\"", label.clone(), package_to_import);
                        let package_file : String = package_to_import[..split_index].to_string();
                        let split_index = split_index + 1;
                        let package_id : String = package_to_import[split_index..].to_string();
                        self.import_dependencies_file(package_file.as_str(), package_id.as_str(), label.clone().as_str());

                    }
                    None => {
                        error!("ERROR_FILE10 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                        panic!("PANIC_FILE10 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                    }
                }
            };
        };
    }

    fn add_package(&mut self, element : Element, file : &str, package : &str, label : String) {
        /*
            Save complete LoadingPackage object in loaded_package

            Input :
            - element (minidom::element::Element) : Element to save
            - file (&str) : file to load in self.input_folder
            - package (&str) : package name wanted in main_file

            Error :
            - none
        */

        // Create full LoadingPackage
        let package_object = LoadingPackage {
            filename : String::from(file),
            id : String::from(package),
            object : element,
            state : LoadingState::Loaded,
        };

        // Save object in hashmap attribute
        self.loaded_package.insert(label.clone(), package_object);

        // Define treatment order
        self.importing_order.insert(label, self.importing_order.len() + 1);
    }

    fn is_package_already_loaded(&self, label : String) -> bool {

                                            // Check if the key is already used
                                            self.loaded_package.contains_key(&label)
    }

    pub fn close(&self) {
        self.file_env.delete_if_empty();
    }
}

impl fmt::Display for LoadingTracker {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result : String = String::new();
        result.push_str("---- LoadingTracker ---");
        result.push_str("\n");
        result.push_str("\n file_env : ");
        result.push_str(format!("{:#?}", &self.file_env).as_str());
        result.push_str("\n importing_order : ");
        result.push_str(format!("{:#?}", &self.importing_order).as_str());
        result.push_str("\n");
        write!(f, "{}", result.as_str())
    }
}

impl LoadingTracker {
    pub fn prebuild(&self, str_file_name : &str) {
        /*

        */

        let mut file_name = self.file_env.get_output_folder();
        file_name.push(str_file_name);
        let mut writing_file = file_name.write_new_file();
        let _ =     write!(writing_file, "#![doc = include_str!(\"../README.md\")]\n\n//! \n\n//! Imported from {:?}\n\n", self.file_env.get_output_folder());
        for (_, package) in &self.loaded_package {
            //writing_file.write_all(&format!("0{:b}", package.get_lowercase_name().into_bytes()));
            let str_element = format!("{:#?}", package.object);
            let _ = write!(writing_file, "mod {} {{\n\n/*\n{}\n*/\n\n}}\n\n", package.get_lowercase_name(), str_element);
        }
    }

    fn _check_lowercase () {

    }
}
