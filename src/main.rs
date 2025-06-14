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
#![doc = include_str!("../README.MD")]

// Shared module
pub mod custom_file_tools;
pub mod custom_log_tools;
pub mod loader_cmof_structure;
pub mod loader_dependencies_explorer;
pub mod loader_deserialise_helper;
pub mod writing_lib_file;
pub mod writing_manager;
pub mod writing_mod_object;

// For "main" use only
mod output_cargo_checker;
mod output_result_manager;

fn main() {
    // // this method needs to be inside main() method
    // std::env::set_var("RUST_BACKTRACE", "1");

    // Settings
    let logger_configuration = "config_log.yml"; // File for configuring logger
    let input_folder = "metamodel_file/"; // Folder where input file can be find
    let main_output_folder = "../Imbriqua_Output_file/"; // Folder dedicased to store output folders and files
    let result_folder = "../Imbriqua_Structure_Result/entities/src"; // Folder dedicased to store output folders and files
    let main_package_file = "BPMNDI.json"; // File of the main package to explore
    let main_package_id = "_0"; // Package ID of main file to explore

    // Initialise global logger, file environment and loading environment
    let _handle = custom_log_tools::open_logger(logger_configuration);
    let file_env = output_result_manager::open_env(input_folder, main_output_folder, result_folder);
    let mut loading_env = loader_dependencies_explorer::open_loader(file_env);
    // Load ordered packages list
    loading_env.prepare(main_package_file, main_package_id, "root");

    // Generate list of class who don't necessite dependencies
    loading_env.writing_preparation();
    // Makin lib.rs file
    loading_env.write_lib_file();
    // Makin all mod_x.rs file
    loading_env.write_mod_object();

    // Cleaning
    loading_env.close();
    // Export the result
    loading_env.export_result();

    // Make doc for loader
    let cargo_loader_package = "Cargo.toml"; // Location of loader environment package Cargo.toml file
    let _loader_link = output_cargo_checker::open_link(cargo_loader_package);
    // assert!(_loader_link.cargo_custom_command(vec!["clean"]));
    // assert!(_loader_link.cargo_custom_command(vec!["doc", "--no-deps"]));
    // assert!(_loader_link.cargo_custom_command(vec!["test"]));

    // Make testing package link
    let cargo_testing_package = "../Imbriqua_Structure_Result/Cargo.toml"; // Location of testing environment package Cargo.toml file
    let _result_link = output_cargo_checker::open_link(cargo_testing_package);
    //assert!(_result_link.cargo_clean());
    // assert!(_result_link.cargo_full_check()); // Make cargo check, test build and doc
}
