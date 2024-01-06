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

mod module_log;
mod module_file_output;
mod module_load_classes;

use log::error;

fn main() {

    // Initialise global logger
    if module_log::open_module().is_err() {
        panic!("Error during the loading on logs modules")
    }

    // Set used folders (input folder and output folder)
    let file_env = match module_file_output::get_folders() {
        Ok(result) => {
            result
        },
        Err(error) => {
            error!("{}", error);
            panic!("Error during the initialisation of input and output folders")
        },
    };

    for file in module_file_output::get_item_list(&file_env).unwrap() {
        println!("{}", file);
    }
    if module_load_classes::run().is_err() {
        panic!("Error during the loading of classes")
    }

    // Delete output folder if is empty
    module_file_output::delete_empty_folders(file_env);

    if false {
        todo!("just, .... TODO !!!")
    }
}
