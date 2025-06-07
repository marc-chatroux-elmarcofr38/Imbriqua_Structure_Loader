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

//#![warn(dead_code)]
#![allow(unreachable_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.MD")]

pub mod module_cmof_structure;
pub mod module_dependencies_explorer;
pub mod module_deserialise_helper;
pub mod module_file_env;
pub mod module_file_manager;
pub mod module_log;
pub mod module_output_checker;
pub mod module_write_control;
pub mod module_write_lib;
pub mod module_write_mods;
pub mod module_write_objects;

fn main() {
    // // this method needs to be inside main() method
    // std::env::set_var("RUST_BACKTRACE", "1");

    // Settings
    let logger_configuration = "config_log.yml"; // File for configuring logger
    let input_folder = "metamodel_file/"; // Folder where input file can be find
    let main_output_folder = "../Output_file/"; // Folder dedicased to store output folders and files
    let main_package_file = "BPMNDI.json"; // File of the main package to explore
    let main_package_id = "_0"; // Package ID of main file to explore
    let cargo_testing_package = "../Imbriqua_Structure_Result/Cargo.toml"; // Location of testing environment package Cargo.toml file

    // Initialise global logger, file environment and loading environment
    let _handle = module_log::open_logger(logger_configuration);
    let file_env = module_file_env::open_env(input_folder, main_output_folder);
    let mut loading_env = module_dependencies_explorer::open_loader(file_env);

    // Load ordered packages list
    loading_env.prepare(main_package_file, main_package_id, "root");
    // Generate list of class who don't necessite dependencies
    loading_env.writing_preparation();
    // Makin lib.rs file
    loading_env.write_lib();
    // Makin all mod_x.rs file
    loading_env.write_mods();
    // Cleaning
    loading_env.close();

    // Make testing package link
    let output_path = loading_env.get_output_folder();
    let link = module_output_checker::open_link(cargo_testing_package);

    // Clean, purge, load and test
    assert!(link.cargo_clean());
    link.purge_source();
    link.load_from(output_path);
    // assert!(link.cargo_full_check());
}
