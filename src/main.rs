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

use chrono::Local;

mod module_log;
mod module_file_output;
mod module_load_classes;

fn main() {

    // Set session name
    let _session_time : String = Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();

    // Initialise global logger
    let (_handle, _config, is_backup) = module_log::open_module();
    if !is_backup {
        return ()
    } 

    // Set used folders (input folder and output folder)
    let file_env = module_file_output::get_folders();
    let iter_file_env = module_file_output::get_item_list(&file_env);

    //
    for (input_file, output_file) in iter_file_env {
        module_load_classes::run(input_file, output_file);
    }

    // Delete output folder if is empty
    module_file_output::delete_empty_folders(file_env);
}
