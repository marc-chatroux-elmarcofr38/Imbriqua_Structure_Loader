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
use crate::module_element_conversion::*;
use crate::module_file_env::*;
use crate::module_file_manager::*;
use crate::module_log::*;

// Dependencies section
// extern crate minidom;
use minidom::Element;
use std::collections::HashMap;
use std::io::Write;

/// Shorcut of __LoadingTracker::new()__, creating LoadingTracker instance using FileEnv object
pub fn open_loader(file_env: FileEnv) -> LoadingTracker {
    LoadingTracker::new(file_env)
}

#[derive(Clone, PartialEq, Debug)]
/// State on package to load
pub enum LoadingState {
    /// No Element (name reserved)
    Empty,
    /// With Element (imported)
    Loaded,
    /// Element used (converted)
    Finished,
}

#[derive(Clone, PartialEq, Debug)]
/// Representation of a package
pub struct LoadingPackage {
    /// Source file of the package
    filename: String,
    /// Source id of the package
    id: String,
    /// Element object of xml content
    object: Element,
    /// State of the package
    state: LoadingState,
}

impl LoadingPackage {
    /// Instanciate a Loading package
    pub fn new(filename: String, id: String) -> Self {
        LoadingPackage {
            filename,
            id,
            object: Element::builder("", "").build(),
            state: LoadingState::Empty,
        }
    }

    /// Lowercase name of the package (no '.', no '#', no uppercase)
    pub fn get_lowercase_name(&self) -> String {
        let str_result = Path::new(&self.filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_ascii_lowercase();
        let str_result = str_result.replace('.', "_");
        let str_result = str_result.replace('#', "_");
        str_result + self.id.as_str().to_ascii_lowercase().as_str()
    }

    /// State of package
    pub fn get_state(&self) -> &LoadingState {
        &self.state
    }

    /// Label used for identification
    pub fn get_label(&self) -> String {
        let mut label = String::from(&self.filename);
        label.push('#');
        label.push_str(&self.id);
        label
    }

    /// Provide 'object' access control
    pub fn get_element(&self) -> &Element {
        if self.state != LoadingState::Loaded {
            panic!()
        }
        &self.object
    }

    /// Save Element and change state
    pub fn make_loaded(&mut self, element: Element) {
        self.object = element;
        self.state = LoadingState::Loaded;
    }

    /// Delete Element and change state
    pub fn make_finished(&mut self) {
        self.object = Element::builder("", "").build();
        self.state = LoadingState::Finished;
    }
}

#[derive(Clone, PartialEq, Debug)]
/// Collection to package loaded, with loading function (load, treatment, export, etc.)
pub struct LoadingTracker {
    /// FileEnv linked with import (input_folder, and output_folder)
    file_env: FileEnv,
    /// Collection of package to import
    pub loaded_package: HashMap<String, LoadingPackage>,
    /// Order of the collection of package
    pub importing_order: HashMap<String, usize>,
}

// Basics
impl LoadingTracker {
    /// Create new instance
    pub fn new(file_env: FileEnv) -> Self {
        LoadingTracker {
            file_env,
            loaded_package: HashMap::new(),
            importing_order: HashMap::new(),
        }
    }

    /// Shortcut function of file_env input folder
    pub fn get_input_folder(&self) -> PathBuf {
        self.file_env.get_input_folder()
    }

    /// Shortcut function of file_env output folder
    pub fn get_output_folder(&self) -> PathBuf {
        self.file_env.get_output_folder()
    }

    /// Shortcut function of file_env output folder
    pub fn get_order_len(&self) -> usize {
        self.importing_order.len()
    }

    /// Prevent circular loading ( A -> B -> A -> ....)
    pub fn check_circular_loading(&self, label: &String) -> bool {
        // Check if the key is already used
        if !self.loaded_package.contains_key(label) {
            false
        // 2nd loading of A -> A already reserved and Empty
        } else {
            *self
                .loaded_package
                .get_key_value(label)
                .unwrap()
                .1
                .get_state()
                == LoadingState::Empty
        }
    }

    /// Prevent multiple loading
    pub fn check_already_loaded(&self, label: &String) -> bool {
        // Check if the key is already used
        self.loaded_package.contains_key(label)
    }

    ///
    pub fn make_finished(&mut self) {
        // Write body
        for key in self.importing_order.keys() {
            let package = self.loaded_package.get_mut(key).unwrap();

            // Change state to 'finished'
            package.make_finished();
        }
    }

    /// Ending loading (delete output folder if empty)
    pub fn close(&self) {
        self.file_env.delete_if_empty();
    }
}

// Algorithm
impl LoadingTracker {
    /// Load minidom element from a gived package, including dependencies, and save element in loaded_package
    pub fn prepare(&mut self, main_file: &str, package_id: &str, parent_label: &str) {
        // Make empty package
        let package = LoadingPackage::new(String::from(main_file), String::from(package_id));
        let label = package.get_label();

        // Check if the loading is necessary
        if self.check_circular_loading(&label) {
            error!("ERROR_DEP01 - Unloaded dependencies : suspicious of circular dependencies ({child} importing {parent})", child=label, parent=parent_label);
            panic!("PANIC_DEP01 - Unloaded dependencies : suspicious of circular dependencies ({child} importing {parent})", child=label, parent=parent_label);
        } else if self.check_already_loaded(&label) {
            debug!("Loading \"{}\" : NOPE : already loaded", label);
            return;
        } else {
            debug!("Loading \"{}\" : START", label);
        }

        // Reserving label in HashMap
        self.loaded_package.insert(package.get_label(), package);

        // Generate file path
        let mut file_path = self.get_input_folder();
        file_path.push(main_file);

        let main_package_element = file_path.get_file_content_as_element();

        // Find "package_id" child
        for child in main_package_element.children() {
            // Package only
            if !child.is("Package", "http://schema.omg.org/spec/MOF/2.0/cmof.xml") {
                continue;
            }
            // With good name
            if child.attr("xmi:id") != Some(package_id) {
                continue;
            }

            // use package_element;
            let mut package_element = child.clone();
            package_element.prefixes = main_package_element.prefixes;

            // Evaluate dependencies, and load it
            self.add_dependencies(&package_element, label.clone());

            // Add package element in loaded_package

            // Save object in hashmap attribute
            let package_object = self.loaded_package.get_mut(&label).unwrap();
            package_object.make_loaded(package_element);

            // Define treatment order
            let max = self.get_order_len();
            self.importing_order.insert(label.clone(), max + 1);

            // End logsl.
            info!("Loading \"{}\" : Finished", label);

            break;
        }
    }

    /// Import dependencies of a package (indirect recursivity from prepare with add_dependencies)
    fn add_dependencies(&mut self, element: &Element, label: String) {
        for child in element.children() {
            if child.is("packageImport", "") {
                // Go to "importedPackage" child
                let imported_package = match child.get_child("importedPackage", "") {
                    Some(result_object) => result_object,
                    None => {
                        error!("ERROR_DEP02 - packageImport element without importedPackage child : package = \"{}\"", label);
                        panic!("PANIC_DEP02 - packageImport element without importedPackage child : package = \"{}\"", label);
                    }
                };

                // Get "href" attribute
                let package_to_import = match imported_package.attr("href") {
                    Some(result_object) => result_object,
                    None => {
                        error!("ERROR_DEP03 - importedPackage element without href attribute : package = \"{}\"", label);
                        panic!("PANIC_DEP03 - importedPackage element without href attribute : package = \"{}\"", label);
                    }
                };

                //
                match package_to_import.find('#') {
                    Some(split_index) => {
                        debug!(
                            "Loading \"{}\" : need to load \"{}\"",
                            label, package_to_import
                        );
                        let package_file: String = package_to_import[..split_index].to_string();
                        let split_index = split_index + 1;
                        let package_id: String = package_to_import[split_index..].to_string();
                        self.prepare(package_file.as_str(), package_id.as_str(), label.as_str());
                    }
                    None => {
                        error!("ERROR_DEP04 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                        panic!("PANIC_DEP04 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                    }
                }
            };
        }
    }

    /// Simple exploration of imported package, exporting unusable file
    pub fn make_primar_result(&mut self, str_file_name: &str) {
        // Get folder path
        let mut file_name = self.get_output_folder();
        file_name.push(str_file_name);
        // Get empty file
        let mut writing_file = file_name.write_new_file();
        // Write head
        let _ = write!(
            writing_file,
            "#![doc = include_str!(\"../README.md\")]\n\n//! \n\n//! Imported from {:?}\n\n",
            self.get_output_folder()
        );
        // Write body
        for (label, package) in &mut self.loaded_package {
            // Logs
            debug!("Working \"{}\" : START", label);
            // Write in a 'mod'
            let element_obj = package.get_element();

            let _ = write!(
                writing_file,
                "mod {package_name} {{\n\n/*\n",
                package_name = package.get_lowercase_name()
            );

            let _ = element_obj.write_to(&mut writing_file);

            let _ = write!(
                writing_file,
                "\n\n{element_as_debug:#?}\n*/\n\n}}\n\n",
                element_as_debug = element_obj
            );

            // Change state to 'finished'
            package.make_finished();

            // Logs
            info!("Working \"{}\" : Finished", label);
        }
    }
    /// Simple exploration of imported package, exporting unusable file
    pub fn make_primar_result_2(&mut self) {
        // lib.rs
        self.make_lib_file_from_package();
        // mod_x.rs
        self.make_mod_file_from_package();
        // Editing LoadingTracker
        self.make_finished();
    }

    /// Make lib.rs from scratch and package
    fn make_lib_file_from_package(&mut self) {
        // Get folder path
        let mut file_name = self.get_output_folder();
        file_name.push("lib.rs");
        // Get empty file
        let mut writing_file = file_name.write_new_file();
        // Write head
        let _ = write!(
            writing_file,
            "#![doc = include_str!(\"../README.md\")]\n\n//! \n\n//! Imported from {:?}\n\n",
            self.get_output_folder()
        );
        // Write body
        for (label, package) in &self.loaded_package {
            // Logs
            debug!("Working \"lib.rs\" from \"{}\" : START", label);
            // Add mod import in main
            let _ = writeln!(writing_file, "pub mod {};", package.get_lowercase_name());
            // Logs
            info!("Working \"lib.rs\" from \"{}\" : Finished", label);
        }
    }

    ///
    fn make_mod_file_from_package(&mut self) {
        // Write body
        for (label, package) in &self.loaded_package {
            // Package output file
            let mut package_output_name = self.get_output_folder();
            package_output_name.push(package.get_lowercase_name() + ".rs");
            let mut writing_package_file = package_output_name.write_new_file();

            // Logs
            debug!("Working \"{}\" : START", label);

            // Write in a 'mod'
            let element_obj = package.get_element();
            let string = String::from(element_obj);
            let _ = writeln!(writing_package_file, "//! {}", package.get_lowercase_name());
            let _ = writeln!(writing_package_file, "//! ");

            // Put XML in Doc
            let _ = writeln!(writing_package_file, "//! ```xml");
            for line in string.lines() {
                let _ = writeln!(writing_package_file, "//! {}", line);
            }
            let _ = writeln!(writing_package_file, "//! ```");

            // Get JSON String
            let mut file_path = self.file_env.get_input_folder();
            let file_name = package.filename.clone();
            let file_name = file_name.replace(".cmof", ".json");
            file_path.push(&file_name);
            let string_content = file_path.get_file_content();

            // Deserialising
            let result_json: FilePackage = serde_json::from_str(&string_content).unwrap();
            let result_json = result_json.packages;
            let string = format!("{:#?}", result_json);

            // Put JSON in Doc
            let _ = writeln!(writing_package_file, "//! ```json");
            for line in string.lines() {
                let _ = writeln!(writing_package_file, "//! {}", line);
            }
            let _ = writeln!(writing_package_file, "//! ```");

            let _ = write!(
                writing_package_file,
                "mod {package_name} {{\n\n/*\n",
                package_name = package.get_lowercase_name()
            );

            let _ = write!(
                writing_package_file,
                "\n\n{element_as_debug:#?}\n*/\n\n}}\n\n",
                element_as_debug = element_obj
            );

            // Logs
            info!("Working \"{}\" : Finished", label);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_log::tests::initialize_log_for_test;

    #[test]
    fn module_dep_01_open_loader() {
        // Logs
        initialize_log_for_test();
        // Setting
        let input_folder = "tests/module_dependencies_explorer/module_dep_01_open_loader/input";
        let main_output_folder =
            "tests/module_dependencies_explorer/module_dep_01_open_loader/output";
        // Preparing
        let file_env = open_env(input_folder, main_output_folder);
        // Test
        let loading_env = open_loader(file_env);
        let _ = loading_env.get_output_folder();
    }
}
