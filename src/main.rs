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

#![doc = include_str!("../README.md")]

mod module_log;
mod module_dependencies_explorer;
mod module_file_manager;
mod module_output_checker;

fn main() {

    // Initialise global logger
    let _handle = module_log::open_module("config_log.yml");

    // Set used folders (input folder and output folder)
    let mut loading_env = module_dependencies_explorer::LoadingTracker::new();

    // Load ordered packages list
    loading_env.import_dependencies_file("BPMNDI.cmof", "_0", "root");

    // info!("{}", loading_env);

    loading_env.prebuild("lib.rs");

    // Delete output folder if is empty
    loading_env.close();

    let result_str : String = loading_env.file_env.output_subfolder;

    let result_package = module_output_checker::PackageLink::from("../Imbriqua_Structure_Result/Cargo.toml");
    result_package.purge();
    result_package.load_from(result_str.as_str());
    if !result_package.cargo_full_check() {panic!()}
    result_package.cargo_clean();
}

/*
#[test]
fn le_test() {

    // Initialise global logger
    let _handle = module_log::open_module();

    // Set used folders (input folder and output folder)
    let loading_env = module_dependencies_explorer::LoadingTracker::new();

    // Delete output folder if is empty
    loading_env.close();
}
*/
