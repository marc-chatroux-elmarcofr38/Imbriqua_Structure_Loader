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
#![doc = include_str!("../doc/loader_dependencies_explorer.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;
use crate::loader_cmof_structure::*;
use crate::output_result_manager::*;

// Dependencies section
use infinitable::Infinitable as UnlimitedNatural;
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;

/// Shorcut of __LoadingTracker::new()__, creating LoadingTracker instance using ResultEnv object
pub fn open_loader(file_env: ResultEnv) -> LoadingTracker {
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
    /// Json
    cmof_object: Option<CMOFPackage>,
    /// State of the package
    state: LoadingState,
}

impl LoadingPackage {
    /// Instanciate a Loading package
    pub fn new(filename: String, id: String) -> Self {
        LoadingPackage {
            filename,
            id,
            cmof_object: None,
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
        // let str_result = str_result.replace('.', "_");
        // let str_result = str_result.replace('#', "_");
        // str_result + self.id.as_str().to_ascii_lowercase().as_str()
        str_result.to_case(Case::Snake)
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
    pub fn get_json(&self) -> &CMOFPackage {
        if self.state != LoadingState::Loaded {
            panic!()
        }
        if self.cmof_object.is_none() {
            panic!()
        }
        self.cmof_object.as_ref().unwrap()
    }

    /// Save Element and change state
    pub fn make_loaded(&mut self, cmof: CMOFPackage) {
        self.cmof_object = Some(cmof);
        self.state = LoadingState::Loaded;
    }

    /// Delete Element and change state
    pub fn make_finished(&mut self) {
        self.cmof_object = None;
        self.state = LoadingState::Finished;
    }
}

#[derive(Clone, PartialEq, Debug)]
/// Representation of a package
pub struct ElementRelation {
    /// Type of the member of the association
    pub element_type: String,
    /// Lower bound for this member
    pub lower: isize,
    /// Upper bound for this member
    pub upper: UnlimitedNatural<usize>,
}

#[derive(Clone, PartialEq, Debug)]
/// Help for AssociationRelation
pub enum RelationPonderationType {
    /// one relation_1 need one relation_2
    OneToOne,
    /// many relation_1 need one relation_2
    OneToMany,
    /// many relation_1 need many relation_2
    ManyToMany,
}

#[derive(Clone, PartialEq, Debug)]
/// Pre Calculation struct helping loading CMOFAssociation
pub struct AssociationRelation {
    /// First relation
    pub relation_1: ElementRelation,
    /// Second relation
    pub relation_2: ElementRelation,
    /// Ponderation of the relation
    pub ponteration_type: RelationPonderationType,
    /// if is itself reference
    pub is_self_referencing: bool,
}

#[derive(Clone, PartialEq, Debug)]
/// Help for naming
pub struct Named {
    /// package lowercase name
    pub package_name: String,
    /// get_technical_name
    pub technical_name: String,
    /// get_table_name
    pub table_name: String,
    /// get_model_name
    pub model_name: String,
    /// get_full_name
    pub full_name: String,
}

#[derive(Clone, PartialEq, Debug)]
/// Help for AssociationRelation pivot
pub enum RankRelation {
    /// Is relation_1
    IsOne,
    /// Is relation_2
    IsSecond,
}

#[derive(Clone, PartialEq, Debug)]
/// List on values necessery for loading but requiring full read of input file for evaluate
pub struct LoadingPreCalculation {
    /// For each owned_member (as model_name format), all name of this package and itself
    /// EX :
    /// "Integer": Named {
    ///     package_name: "dc",
    ///     technical_name: "DC.cmof#Integer",
    ///     table_name: "dc_integer",
    ///     model_name: "Integer",
    ///     full_name: "dc_primitive_integer",
    /// },
    /// "ACorrelationKeyRefCorrelationSubscription": Named {
    ///     package_name: "bpmn_20",
    ///     technical_name: "BPMN20.cmof#A_correlationKeyRef_correlationSubscription",
    ///     table_name: "bpmn_20_a_correlation_key_ref_correlation_subscription",
    ///     model_name: "ACorrelationKeyRefCorrelationSubscription",
    ///     full_name: "bpmn_20_association_a_correlation_key_ref_correlation_subscription",
    /// },
    pub owned_member_type_list: HashMap<String, Named>,
    /// For each CMOFEnumeration (as model_name format), this default value, loaded from "metamodel_file_extension/enumeration_default_value.json"
    /// EX :
    ///     "ParticipantBandKind": "TopInitiating",
    ///     "MultiInstanceBehavior": "All",
    pub enumeration_default_value: HashMap<String, String>,
    /// For each CMOFPrimitiveType (as model_name format), this Rust equivalent type, loaded from "metamodel_file_extension/primitive_type_conversion.json"
    /// EX :
    ///     "ParticipantBandKind": "TopInitiating",
    ///     "MultiInstanceBehavior": "All",
    pub primitive_type_conversion: HashMap<String, String>,
    /// For each CMOFassociation (as model_name format), the linked AssociationRelation object
    /// EX :
    /// "A_inputDataRef_inputOutputBinding": AssociationRelation {
    ///     relation_1: ElementRelation {
    ///         element_type: "InputOutputBinding",
    ///         lower: 0,
    ///         upper: Infinity,
    ///     },
    ///     relation_2: ElementRelation {
    ///         element_type: "InputSet",
    ///         lower: 1,
    ///         upper: Finite(
    ///             1,
    ///         ),
    ///     },
    ///     ponteration_type: OneToMany,
    ///     is_self_referencing: false,
    /// },
    pub association_relation: HashMap<String, AssociationRelation>,
    /// For each CMOFClass (as model_name format), all associed CMOFAssociation with rank (provided by association_relation)
    /// EX :
    /// "CorrelationProperty": [
    ///     (
    ///         "A_correlationPropertyRetrievalExpression_correlationproperty",
    ///         IsSecond,
    ///     ),
    ///     (
    ///         "A_type_correlationProperty",
    ///         IsOne,
    ///     ),
    ///     (
    ///         "A_correlationPropertyRef_correlationPropertyBinding",
    ///         IsSecond,
    ///     ),
    ///     (
    ///         "A_correlationPropertyRef_correlationKey",
    ///         IsSecond,
    ///     ),
    /// ],
    pub association_relation_by_class: HashMap<String, Vec<(String, RankRelation)>>,
    /// For each CMOFClass (as model_name format), all CMOFClass (as model_name format) who use it as "Super"
    pub reverse_super_link: HashMap<String, Vec<String>>,
}
impl LoadingPreCalculation {
    /// Create new instance
    pub fn new() -> Self {
        LoadingPreCalculation {
            owned_member_type_list: HashMap::new(),
            enumeration_default_value: HashMap::new(),
            primitive_type_conversion: HashMap::new(),
            association_relation: HashMap::new(),
            association_relation_by_class: HashMap::new(),
            reverse_super_link: HashMap::new(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
/// Collection to package loaded, with loading function (load, treatment, export, etc.)
pub struct LoadingTracker {
    /// ResultEnv linked with import (input_folder, and output_folder)
    file_env: ResultEnv,
    /// Collection of package to import
    loaded_package: HashMap<String, LoadingPackage>,
    /// Order of the collection of package
    pub importing_order: BTreeMap<usize, String>,
    /// builing pre calculation result
    pub pre_calculation: LoadingPreCalculation,
}

// Basics
impl LoadingTracker {
    /// Create new instance
    pub fn new(file_env: ResultEnv) -> Self {
        LoadingTracker {
            file_env,
            loaded_package: HashMap::new(),
            importing_order: BTreeMap::new(),
            pre_calculation: LoadingPreCalculation::new(),
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

    /// Cleaning end process
    pub fn close(&mut self) {
        // Lock all package
        for package in self.loaded_package.values_mut() {
            // Change state to 'finished'
            package.make_finished();
        }
        // Ending loading (delete output folder if empty)
        self.file_env.delete_if_empty();
    }

    /// Recall for copy output to result
    pub fn export_result(&mut self) {
        // Copy output to result
        self.file_env.export_result();
    }
}

// Algorithm
impl LoadingTracker {
    ///
    pub fn get_package_in_order(&self) -> Vec<(&String, &LoadingPackage)> {
        let mut result: HashMap<&String, &LoadingPackage> = HashMap::new();
        debug!("{:?}", &self.importing_order);
        for (_, value) in &self.importing_order {
            if self.loaded_package.get(value).is_some() {
                result.insert(&value, self.loaded_package.get(value).unwrap());
            }
        }
        let mut v = Vec::from_iter(result);
        v.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        v
    }

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
        let string_content = file_path.get_file_content();

        // Deserialising
        let cmof_result: FilePackage = serde_json::from_str(&string_content).unwrap();
        let cmof_package = cmof_result.package;

        // Checi ID
        if cmof_package.xmi_id != package_id {
            panic!()
        }

        // Evaluate dependencies, and load it
        self.add_dependencies(&cmof_package, label.clone());

        // Save object in hashmap attribute
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
    }

    /// Import dependencies of a package (indirect recursivity from prepare with add_dependencies)
    fn add_dependencies(&mut self, cmof_package: &CMOFPackage, label: String) {
        for child in cmof_package.package_import.iter() {
            // Go to "importedPackage" child
            match child {
                EnumPackageImport::PackageImport(content) => {
                    match &content.imported_package {
                        EnumImportedPackage::ImportedPackage(content_2) => {
                            let package_to_import = content_2.href.clone();

                            //
                            match package_to_import.find('#') {
                                Some(split_index) => {
                                    debug!(
                                        "Loading \"{}\" : need to load \"{}\"",
                                        label, package_to_import
                                    );
                                    let package_file: String =
                                        package_to_import[..split_index].to_string();
                                    let package_file: String =
                                        package_file.replace(".cmof", ".json");
                                    let split_index = split_index + 1;
                                    let package_id: String =
                                        package_to_import[split_index..].to_string();
                                    self.prepare(
                                        package_file.as_str(),
                                        package_id.as_str(),
                                        label.as_str(),
                                    );
                                }
                                None => {
                                    error!("ERROR_DEP04 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                                    panic!("PANIC_DEP04 - href attribute without '#' separator : package = \"{}\", href = \"{}\"", label, package_to_import);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn loader_dependencies_explorer_01_open_loader() {
        // Logs
        initialize_log_for_test();
        // Setting
        let input_folder =
            "tests/loader_dependencies_explorer/loader_dependencies_explorer_01_open_loader/input";
        let main_output_folder =
            "tests/loader_dependencies_explorer/loader_dependencies_explorer_01_open_loader/output";
        let result_folder =
            "tests/loader_dependencies_explorer/loader_dependencies_explorer_01_open_loader/result";
        // Preparing
        let file_env = open_env(input_folder, main_output_folder, result_folder);
        // Test
        let loading_env = open_loader(file_env);
        let _ = loading_env.get_output_folder();
    }
}

/// Bullshit function : define if a type (represent as string.....) involve to set a structure using reference ("&")
pub fn is_lifetime_dpt(input: &str) -> bool {
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

#[derive(Clone, PartialEq, Debug)]
/// State on package to load
pub enum ClassClassification {
    /// Primal : depend of nothing
    Primal,
    /// Simple : don't need lifetime for utilization, but need reference
    Simple,
    /// Complex : need lifetime for utilization
    Complex,
}
