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
#![doc = include_str!("../README.MD")]

pub mod module_log;
pub mod module_dependencies_explorer;
pub mod module_file_env;
pub mod module_file_manager;
pub mod module_output_checker;

fn main() {

    // Settings
    let logger_configuration = "config_log.yml";                            // File for configuring logger
    let input_folder = "metamodel_file/";                                   // Folder where input file are stored
    let main_output_folder = "../Output_file/";                             // Folder containing output folders and files
    let main_package_file = "BPMNDI.cmof";                                  // File containing the package to explore
    let main_package_id = "_0";                                             // Package ID of main file to explore
    let cargo_testing_package = "../Imbriqua_Structure_Result/Cargo.toml";  // Location of testing environment package Cargo.toml file

    // Initialise global logger, file environment and loading environment
    let _handle = module_log::open_logger(logger_configuration);
    let file_env = module_file_env::open_env(input_folder, main_output_folder);
    let mut loading_env = module_dependencies_explorer::LoadingTracker::new(file_env);

    // Load ordered packages list
    loading_env.import_dependencies_file(&main_package_file, main_package_id, "root");
    loading_env.prebuild("lib.rs");

    // Delete output folder if is empty
    // loading_env.close();

    let result_path = loading_env.file_env.get_output_folder();

    let result_package = module_output_checker::PackageLink::from(cargo_testing_package);
    result_package.cargo_clean();
    result_package.purge();
    result_package.load_from(result_path);
    if !result_package.cargo_full_check() {panic!()}
}


/*
#[test]
fn le_test() {

    // Initialise global logger
    let _handle = module_log::open_logger();

    // Set used folders (input folder and output folder)
    let loading_env = module_dependencies_explorer::LoadingTracker::new();

    // Delete output folder if is empty
    loading_env.close();
}
*/
