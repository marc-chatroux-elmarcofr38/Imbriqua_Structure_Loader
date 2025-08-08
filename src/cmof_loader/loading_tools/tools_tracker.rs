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

// Package section
use crate::cmof_loader::*;
use crate::output_result_manager::*;

// Dependencies section

// ####################################################################################################
//
// ####################################################################################################

#[derive(Clone, Debug)]
/// Collection to package loaded, with loading function (load, treatment, export, etc.)
pub struct LoadingTracker {
    /// ResultEnv linked with import (input_folder, and output_folder)
    file_env: ResultEnv,
    /// Collection of package to import
    loaded_package: BTreeMap<String, LoadingPackage>,
    /// Order of the collection of package
    pub importing_order: BTreeMap<usize, String>,
}

// Basics
impl LoadingTracker {
    /// Create new instance
    pub fn new(file_env: ResultEnv) -> Result<Self, anyhow::Error> {
        Ok(LoadingTracker {
            file_env,
            loaded_package: BTreeMap::new(),
            importing_order: BTreeMap::new(),
        })
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

    /// Cleaning end process
    pub fn close(&mut self) -> Result<(), anyhow::Error> {
        // Lock all package
        for package in self.loaded_package.values_mut() {
            // Change state to 'finished'
            package.make_finished()?;
        }
        // Ending loading (delete output folder if empty)
        self.file_env.delete_if_empty()?;
        Ok(())
    }

    /// Recall for copy output to result
    pub fn export_result(&mut self) -> Result<(), anyhow::Error> {
        // Copy output to result
        self.file_env.export_result()?;
        Ok(())
    }
}

// Algorithm
impl LoadingTracker {
    ///
    pub fn get_package_in_order(&self) -> BTreeMap<String, &LoadingPackage> {
        let mut result: BTreeMap<String, &LoadingPackage> = BTreeMap::new();
        debug!("get_package_in_order : {:?}", &self.importing_order);
        for (_, value) in &self.importing_order {
            if self.loaded_package.get(value).is_some() {
                result.insert(value.clone(), self.loaded_package.get(value).unwrap());
            }
        }
        result
    }

    /// Load minidom element from a gived package, including dependencies, and save element in loaded_package
    pub fn make_prepare(
        &mut self,
        main_file: &str,
        package_id: &str,
        parent_label: &str,
    ) -> Result<(), anyhow::Error> {
        // Load
        let r = self.prepare(main_file, package_id, parent_label);
        catch_error_and_log(r, &self)?;
        // Create dict for collect_object and make_post_deserialize
        let mut dict_setting: BTreeMap<String, String> = BTreeMap::new();
        let mut dict_object: BTreeMap<String, EnumCMOF> = BTreeMap::new();
        // Collect
        let r = self.collect_object(&mut dict_setting, &mut dict_object);
        catch_error_and_log(r, &self)?;
        // Make post deserialize
        let r = self.make_post_deserialize(&mut dict_object);
        catch_error_and_log(r, &self)?;
        for (_, x) in &dict_object {
            match x {
                EnumCMOF::CMOFClass(class) => {
                    class.generate_reverse_super_class(&dict_object)?;
                    class.generate_relation(&dict_object)?;
                }
                _ => {}
            }
        }
        // Debug (trace  level)
        trace!("Self after collect_object : {:#?}", self);
        trace!("dict_setting after collect_object : {:#?}", dict_setting);
        trace!(
            "COUNT of dict_setting after collect_object : {}",
            dict_setting.len()
        );
        trace!(
            "COUNT of dict_object after collect_object : {}",
            dict_object.len()
        );
        trace!("dict_object after collect_object : {:#?}", dict_object);
        Ok(())
    }

    /// Load minidom element from a gived package, including dependencies, and save element in loaded_package
    fn prepare(
        &mut self,
        main_file: &str,
        package_id: &str,
        parent_label: &str,
    ) -> Result<(), anyhow::Error> {
        // Make empty package
        let package = LoadingPackage::new(String::from(main_file), String::from(package_id));
        let label = package.get_label();

        // Check if the loading is necessary
        if self.check_circular_loading(&label) {
            error!("ERROR_DEP01 - Unloaded dependencies : suspicious of circular dependencies ({child} importing {parent})", child=label, parent=parent_label);
            panic!("PANIC_DEP01 - Unloaded dependencies : suspicious of circular dependencies ({child} importing {parent})", child=label, parent=parent_label);
        } else if self.check_already_loaded(&label) {
            debug!("Loading \"{}\" : NOPE : already loaded", label);
            return Ok(());
        } else {
            debug!("Loading \"{}\" : START", label);
        }

        // Reserving label in BTreeMap
        self.loaded_package.insert(package.get_label(), package);

        // Generate file path
        let mut file_path = self.get_input_folder();
        file_path.push(main_file);
        let string_content = file_path.get_file_content()?;

        // Deserialising
        let cmof_result: FilePackage = serde_json::from_slice(&string_content.as_bytes()).unwrap();
        let cmof_package = cmof_result.package;

        // Check ID
        if cmof_package.xmi_id.get_object_id() != package_id {
            error!("{} / {}", package_id, cmof_package.xmi_id.get_object_id());
            panic!()
        }

        // Evaluate dependencies, and load it
        self.add_dependencies(&cmof_package, label.clone())?;

        // Save object in BTreeMap attribute
        let package_object = self.loaded_package.get_mut(&label).unwrap();
        // package_object.make_loaded_element(package_element);
        package_object.make_loaded(cmof_package);

        // Define treatment order
        let max = self.get_order_len();
        self.importing_order.insert(max + 1, label.clone());

        // End logsl.
        info!("Preparing \"{}\" : Finished", label);

        //     break;
        // }
        Ok(())
    }

    /// Import dependencies of a package (indirect recursivity from prepare with add_dependencies)
    fn add_dependencies(
        &mut self,
        cmof_package: &CMOFPackage,
        label: String,
    ) -> Result<(), anyhow::Error> {
        for (_, child) in cmof_package.package_import.iter() {
            // Go to "importedPackage" child
            match child {
                EnumPackageImport::PackageImport(content) => match &content.imported_package {
                    EnumImportedPackage::ImportedPackage(content_2) => {
                        let package_to_import = content_2.href.clone();
                        debug!(
                            "Loading \"{}\" : need to load \"{}\"",
                            label,
                            package_to_import.get_package_id()
                        );
                        let mut package_file: String = package_to_import.get_package_id().clone();
                        package_file.push_str(".json");
                        let package_id: String = package_to_import.get_object_id().clone();
                        self.prepare(package_file.as_str(), package_id.as_str(), label.as_str())?;
                    }
                },
            }
        }
        Ok(())
    }
}

impl SetCMOFTools for LoadingTracker {
    fn collect_object(
        &mut self,
        dict_setting: &mut BTreeMap<String, String>,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        for (_, p) in &mut self.loaded_package {
            p.collect_object(dict_setting, dict_object)?;
        }
        Ok(())
    }

    fn make_post_deserialize(
        &self,
        dict_object: &mut BTreeMap<String, EnumCMOF>,
    ) -> Result<(), anyhow::Error> {
        for (_, p) in &self.loaded_package {
            p.make_post_deserialize(dict_object)?;
        }
        Ok(())
    }
}

// ####################################################################################################
//
// ####################################################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn test_01_creation() {
        fn test() -> Result<(), anyhow::Error> {
            initialize_log_for_test();

            panic!();

            Ok(())
        }

        let r = test();
        assert!(r.is_ok());
    }
}
