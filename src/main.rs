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
mod module_dependencies_explorer;
use log::info;

fn main() {

    // Initialise global logger
    let (_handle, _config, _is_backup) = module_log::open_module();

    // Set used folders (input folder and output folder)
    let mut loading_env = module_dependencies_explorer::LoadingTracker::new();

    // Load ordered packages list
    loading_env.import_dependencies_file("BPMNDI.cmof", "_0", "root");

    info!("{}", loading_env);

    loading_env.prebuild();

    // Delete output folder if is empty
    loading_env.close();
}
