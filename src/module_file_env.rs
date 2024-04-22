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
#![doc = include_str!("../doc/module_file_env.md")]

// Package section
use crate::module_log::*;
use crate::module_file_manager::{FileManager, PathBuf, Path};

// Dependencies section
use chrono::Local;

/// Provide management of a input folder and a output folder (created with time name)
#[derive(Clone, PartialEq, Debug)]
pub struct FileEnv {
    /// Input folder, obtainable with __get_input_folder()__
    input_folder : PathBuf,
    /// Output folder (chilf of main_output_folder, named with time-formatting), obtainable with __get_output_folder()__
    output_folder : PathBuf,
}

impl FileEnv {

    fn new(input_folder : &str, main_output_folder : &str) -> Self {
        //Set input folder path and output folder path
        let path_input_folder : PathBuf = Path::new(input_folder).canonicalize_pathbuf();
        let mut path_output_folder : PathBuf = Path::new(main_output_folder).canonicalize_pathbuf();
        let time_string : String = Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();
        path_output_folder.push(time_string);
        // Create folder
        path_output_folder.create_folder();
        // Checking instance
        path_input_folder.check_is_dir();
        path_output_folder.check_is_dir();
        // Create instance
        let result = FileEnv {
            input_folder : path_input_folder,
            output_folder : path_output_folder,
        };
        // Return result
        result
    }

    /// Deleting output folder if empty (for cleaning output main folder)
    pub fn delete_if_empty (&self) {
        self.output_folder.delete_folder(true);
    }

    /// Read input folder Path as PathBuf
    pub fn get_input_folder(&self) -> PathBuf {
        self.input_folder.clone()
    }

    /// Read output folder Path as PathBuf
    pub fn get_output_folder(&self) -> PathBuf {
        self.output_folder.clone()
    }
}

/// Shorcut of __FileEnv::new()__, creating FileEnv instance and creating output folder with time name
pub fn open_env(input_folder : &str, main_output_folder : &str) -> FileEnv {
    FileEnv::new(input_folder, main_output_folder)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::module_log::tests::initialize_log_for_test;

    #[test]
    fn module_env_01_open_env() {
        // Logs
        initialize_log_for_test();
        // Setting
        let input_path = "tests/module_file_env/module_env_01_open_env/input";
        let main_output_path = "tests/module_file_env/module_env_01_open_env/main_output";
        // Preparing
        Path::new(main_output_path).purge_folder();
        // Test
        let _ = open_env(input_path, main_output_path);
    }

    #[ignore]
    fn _test_template() {
        // Logs
        initialize_log_for_test();
        // Setting
        // Preparing
        // Test
    }
}