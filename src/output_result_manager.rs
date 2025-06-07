/*
Copyright 2023-2024 CHATROUX MARC

This file is part of Imbriqua Structure, a interpreter of BPMN model files (in UML notation) for
Imbriqua Engine project
s
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
#![doc = include_str!("../doc/output_result_manager.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

// Dependencies section
use chrono::Local;

/// Provide management of a input folder and a output folder (created with time name)
#[derive(Clone, PartialEq, Debug)]
pub struct ResultEnv {
    /// Input folder, obtainable with __get_input_folder()__
    input_folder: PathBuf,
    /// Output folder (child of parent_output_folder, named with time-formatting), obtainable with __get_output_folder()__
    output_folder: PathBuf,
    /// Result folder, obtainable with __get_result_folder()__
    result_folder: PathBuf,
}

impl ResultEnv {
    fn new(input_folder: &str, parent_output_folder: &str, result_folder: &str) -> Self {
        // Create output folder (prevention)
        Path::new(parent_output_folder).create_folder();
        Path::new(result_folder).create_folder();

        // Checking instance
        Path::new(input_folder).check_is_dir();
        Path::new(parent_output_folder).check_is_dir();
        Path::new(result_folder).check_is_dir();

        //Set input folder path and output folder path
        let path_input_folder: PathBuf = Path::new(input_folder).canonicalize_pathbuf();
        let mut path_output_folder: PathBuf =
            Path::new(parent_output_folder).canonicalize_pathbuf();
        let time_string: String = Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();
        path_output_folder.push(time_string);
        let path_result_folder: PathBuf = Path::new(result_folder).canonicalize_pathbuf();

        // Create sub output folder
        path_output_folder.create_folder();

        // Logs
        info!(
            "ResultEnvironment setting with {:?} input folder",
            path_input_folder
        );
        info!(
            "ResultEnvironment setting with {:?} output folder",
            path_output_folder
        );
        info!(
            "ResultEnvironment setting with {:?} result folder",
            path_result_folder
        );

        // Create instance
        ResultEnv {
            input_folder: path_input_folder,
            output_folder: path_output_folder,
            result_folder: path_result_folder,
        }
    }

    /// Read input folder Path as PathBuf
    pub fn get_input_folder(&self) -> PathBuf {
        self.input_folder.clone()
    }

    /// Read output folder Path as PathBuf
    pub fn get_output_folder(&self) -> PathBuf {
        self.output_folder.clone()
    }

    /// Read result folder Path as PathBuf
    pub fn get_result_folder(&self) -> PathBuf {
        self.result_folder.clone()
    }

    /// Deleting output folder if empty (for cleaning output main folder)
    pub fn delete_if_empty(&self) {
        self.output_folder.delete_folder(true);
    }

    /// Copy output to result
    pub fn export_result(&self) {
        // Purge folder
        self.result_folder.purge_folder();
        // Copy content of output_folder to result_folder
        self.output_folder.copy_folder(&self.result_folder);
    }
}

/// Shorcut of __ResultEnv::new()__, creating ResultEnv instance and creating output folder with time name
pub fn open_env(input_folder: &str, main_output_folder: &str, result_folder: &str) -> ResultEnv {
    ResultEnv::new(input_folder, main_output_folder, result_folder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn module_env_01_open_env() {
        // Logs
        initialize_log_for_test();
        // Setting
        let input_path = "tests/module_file_env/module_env_01_open_env/input";
        let main_output_path = "tests/module_file_env/module_env_01_open_env/main_output";
        let result_path = "tests/module_file_env/module_env_01_open_env/result_output";
        // Preparing
        Path::new(main_output_path).purge_folder();
        // Test
        let _ = open_env(input_path, main_output_path, result_path);
    }

    #[test]
    fn module_env_02_detele_if_empty() {
        // Logs
        initialize_log_for_test();
        // Setting
        let input_path = "tests/module_file_env/module_env_02_detele_if_empty/input";
        let main_output_path = "tests/module_file_env/module_env_02_detele_if_empty/main_output";
        let result_path = "tests/module_file_env/module_env_02_detele_if_empty/result";
        // Preparing
        Path::new(main_output_path).purge_folder();
        // Test
        let file_env = open_env(input_path, main_output_path, result_path);
        file_env.delete_if_empty();
    }

    #[test]
    fn module_env_03_get_input_folder() {
        // Logs
        initialize_log_for_test();
        // Setting
        let input_path = "tests/module_file_env/module_env_03_get_folders/input";
        let main_output_path = "tests/module_file_env/module_env_03_get_folders/main_output";
        let result_path = "tests/module_file_env/module_env_03_get_folders/result";
        // Preparing
        Path::new(main_output_path).purge_folder();
        // Test
        let file_env = open_env(input_path, main_output_path, result_path);
        file_env.get_input_folder();
        file_env.get_output_folder();
        file_env.get_result_folder();
    }

    #[test]
    fn module_env_04_export_result() {
        // Logs
        initialize_log_for_test();
        // Setting
        let step_1 = "tests/module_file_env/module_env_04_export_result/step_1";
        let step_2 = "tests/module_file_env/module_env_04_export_result/step_2";
        let input_path = "tests/module_file_env/module_env_04_export_result/input";
        let main_output_path = "tests/module_file_env/module_env_04_export_result/main_output";
        let result_path = "tests/module_file_env/module_env_04_export_result/result";
        // Preparing
        Path::new(main_output_path).purge_folder();
        // Test
        let file_env = open_env(input_path, main_output_path, result_path);
        // Clone input in output
        Path::new(step_1).copy_folder(&file_env.get_output_folder());
        Path::new(step_2).copy_folder(&file_env.get_result_folder());
        // Export output in result
        file_env.export_result();
        // Check if all files are in result
        let path_test_1 =
            Path::new("tests/module_file_env/module_env_04_export_result/result/f1.txt");
        let path_test_2 = Path::new("tests/module_file_env/module_env_04_export_result/result/f2");
        let path_test_3 =
            Path::new("tests/module_file_env/module_env_04_export_result/result/f2/f3.txt");
        assert!(path_test_1.exists());
        assert!(path_test_2.exists());
        assert!(path_test_3.exists());
        let path_test_1 =
            Path::new("tests/module_file_env/module_env_04_export_result/result/f4.txt");
        let path_test_2 = Path::new("tests/module_file_env/module_env_04_export_result/result/f4");
        let path_test_3 =
            Path::new("tests/module_file_env/module_env_04_export_result/result/f5/f6.txt");
        assert!(!path_test_1.exists());
        assert!(!path_test_2.exists());
        assert!(!path_test_3.exists());
        // Ending
        Path::new(main_output_path).purge_folder();
        Path::new(result_path).purge_folder();
    }
}
