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
#![allow(unused)]
#![doc = include_str!("../doc/output_cargo_checker.md")]

// Package section
use crate::custom_file_tools::*;
use crate::custom_log_tools::*;

// Dependencies section
use std::process::Command;

/// Represent a cargo package folder link, used to checking metacode result
///
/// See output_cargo_checker for examples and errors informations
#[derive(Clone, PartialEq, Debug)]
pub struct PackageLink {
    /// PathBuf of the Cargo.toml file of the package
    absolute_path: PathBuf,
}

/// Shorcut of PackageLink::from()__, creating PackageLink instance for cargo checking
pub fn open_link(str_relative_cargo_path: &str) -> PackageLink {
    PackageLink::from(str_relative_cargo_path)
}

impl PackageLink {
    /// Initialise a cargo package folder link from relative string path
    ///
    /// str_relative_cargo_path (&str) : relative path of the Cargo.toml file to link
    pub fn from(str_relative_cargo_path: &str) -> Self {
        // Make it absolute
        let var_absolute_path = Path::new(str_relative_cargo_path).canonicalize_pathbuf();
        // Instanciate object
        let result = PackageLink {
            absolute_path: var_absolute_path,
        };
        // Checking file integrity
        result.cargo_integrity_check();
        // Return
        result
    }

    /// Get absolute_path attribute
    ///
    /// result (PathBuf) : Path of the Cargo.toml file of the package (absolute path)
    pub fn get_absolute_cargo_path(&self) -> PathBuf {
        self.absolute_path.clone()
    }

    /// Get 'src/' folder path, calculate using src subfolder of Cargo.toml parent folder
    ///
    /// result (PathBuf) : Path of the src/ folder of the package (absolute path)
    pub fn get_absolute_source_path(&self) -> PathBuf {
        let mut result = self.absolute_path.clone();
        result.pop();
        result.push("src");
        result
    }

    /// Running cargo custom command
    ///
    /// args (Vec<&str>) : list of arg to forward to the command
    ///
    /// # Errors
    ///
    /// See output_cargo_checker documentation page for errors details
    ///
    /// # Examples
    ///
    /// See output_cargo_checker documentation page for example
    pub fn cargo_custom_command(&self, args: Vec<&str>) -> bool {
        // Instantiate command
        let mut cargo_1 = Command::new("cargo");
        let _ = cargo_1
            .args(args)
            .arg(format!(
                "--manifest-path={}",
                self.absolute_path.to_string_lossy()
            ))
            .output()
            .expect("process failed to execute");
        // Logging debug and getting succes result
        let result_1 = represent_command_output(&mut cargo_1).is_some_and(|x| x);
        // Logging succes result
        trace!(
            "Running cargo command : {} : {:?}",
            if result_1 { "succes" } else { "error" },
            cargo_1
        );

        result_1
    }

    /// Running cargo full check (check, test, build, doc)
    ///
    /// # Errors
    ///
    /// See output_cargo_checker documentation page for errors details
    ///
    /// # Examples
    ///
    /// See output_cargo_checker documentation page for examples
    pub fn cargo_full_check(&self) -> bool {
        // Running 'cargo check --all-features'
        let res_1 = self.cargo_custom_command(vec!["check", "--all-features"]);
        // Running 'cargo test --all-features --no-run'
        let res_2 = self.cargo_custom_command(vec!["test", "--all-features", "--no-run"]);
        // Running 'cargo build --all-features'
        let res_3 = self.cargo_custom_command(vec!["build", "--all-features"]);
        // Running 'cargo doc --no-deps'
        let res_4 = self.cargo_custom_command(vec!["doc", "--no-deps"]);
        // Get succes
        res_1 && res_2 && res_3 && res_4
    }

    /// Running cargo locate-project command, allowing to check existence of Cargo.toml file
    ///
    /// # Errors
    ///
    /// See output_cargo_checker documentation page for errors details
    ///
    /// # Examples
    ///
    /// See output_cargo_checker documentation page for examples
    pub fn cargo_integrity_check(&self) -> bool {
        // Running 'cargo locate-project'
        let res_1 = &self.cargo_custom_command(vec!["locate-project"]);

        match res_1 {
            true => {
                info!("Cargo.toml exist for {:?}", &self.absolute_path);
                true
            }
            false => {
                error!(
                    "PANIC_OUT01 - Can't find cargo.toml {:?}",
                    &self.absolute_path
                );
                panic!(
                    "PANIC_OUT01 - Can't find cargo.toml {:?}",
                    &self.absolute_path
                );
            }
        }
    }

    /// Running cargo clean command, allowing to purge "target" folder
    ///
    /// # Errors
    ///
    /// See output_cargo_checker documentation page for errors details
    ///
    /// # Examples
    ///
    /// See output_cargo_checker documentation page for examples
    pub fn cargo_clean(&self) -> bool {
        match &self.cargo_custom_command(vec!["clean"]) {
            true => {
                info!("Cleaning {:?} package", &self.absolute_path);
                true
            }
            false => {
                error!("PANIC_OUT02 - Can't clean {:?}", &self.absolute_path);
                panic!("PANIC_OUT02 - Can't clean {:?}", &self.absolute_path);
            }
        }
    }
}

/// Printing command result, used by __check_result__ function
///
/// # Errors
///
/// See output_cargo_checker documentation page for errors details
///
/// # Examples
///
/// See output_cargo_checker documentation page for examples
fn represent_command_output(command: &mut Command) -> Option<bool> {
    // Result catch
    let command_output = match command.output() {
        Ok(result) => result,
        Err(error) => {
            warn!(
                "WARN_OUT01 - Cound't get output of {:?} - {}",
                &command, error
            );
            return None;
        }
    };
    // Result catch
    let str_stdout = match std::str::from_utf8(&command_output.stdout) {
        Ok(result) => result,
        Err(error) => {
            warn!(
                "WARN_OUT02 - Couldn't get STDOUT of {:?} - {}",
                &command, error
            );
            return None;
        }
    };
    // Result catch
    let str_stderr = match std::str::from_utf8(&command_output.stderr) {
        Ok(result) => result,
        Err(error) => {
            warn!(
                "WARN_OUT03 - Couldn't get STDERR of {:?} - {}",
                &command, error
            );
            return None;
        }
    };
    // Logs
    let command_str = format!("{:?}", &command);
    let command_str = command_str.replace("\" \"", " ");
    debug!(
        "\nsucces:\n{}\ncommand:\n{:#?}\nstdout:\n{}\nstderr:\n{}",
        command_output.status.success(),
        command_str,
        str_stdout,
        str_stderr
    );
    info!(
        "result {} for {}",
        command_output.status.success(),
        command_str
    );
    // Boolean result of succes
    Some(command_output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::custom_log_tools::tests::initialize_log_for_test;

    #[test]
    fn output_cargo_checker_01_from() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder = "tests/output_cargo_checker/output_cargo_checker_01_from/project_b/Cargo.toml";
        // Preparing
        // Test
        let _ = open_link(folder);
    }

    #[test]
    fn output_cargo_checker_02_get_absolute_cargo_path() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder = "tests/output_cargo_checker/output_cargo_checker_02_get_absolute_cargo_path/project_b/Cargo.toml";
        let good_result = Path::new("tests/output_cargo_checker/output_cargo_checker_02_get_absolute_cargo_path/project_b/Cargo.toml");
        // Preparing
        // Test
        let package_link = open_link(folder);
        let result = package_link.get_absolute_cargo_path();
        assert_eq!(result, good_result.canonicalize_pathbuf());
    }

    #[test]
    fn output_cargo_checker_03_get_absolute_source_path() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder = "tests/output_cargo_checker/output_cargo_checker_03_get_absolute_source_path/project_b/Cargo.toml";
        let good_result = Path::new(
            "tests/output_cargo_checker/output_cargo_checker_03_get_absolute_source_path/project_b/src",
        );
        // Preparing
        // Test
        let package_link = open_link(folder);
        let result = package_link.get_absolute_source_path();
        assert_eq!(result, good_result.canonicalize_pathbuf());
    }

    #[test]
    fn output_cargo_checker_04_cargo_custom_command() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder =
            "tests/output_cargo_checker/output_cargo_checker_04_cargo_custom_command/project_b/Cargo.toml";
        let doc = Path::new(
            "tests/output_cargo_checker/output_cargo_checker_04_cargo_custom_command/project_b/target/doc",
        );
        // Preparing
        if doc.exists() {
            doc.delete_folder(false);
        }
        assert!(!doc.exists());
        // Test
        let package_link = open_link(folder);
        let result = package_link.cargo_custom_command(vec!["doc"]);
        assert!(result);
        assert!(doc.exists());
        package_link.cargo_clean();
    }

    #[test]
    fn output_cargo_checker_05_cargo_full_check() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder =
            "tests/output_cargo_checker/output_cargo_checker_05_cargo_full_check/project_b/Cargo.toml";
        // Preparing
        // Test
        let package_link = open_link(folder);
        let result = package_link.cargo_full_check();
        assert!(result);
        package_link.cargo_clean();
    }

    #[test]
    fn output_cargo_checker_06_cargo_integrity_check() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder =
            "tests/output_cargo_checker/output_cargo_checker_06_cargo_integrity_check/project_b/Cargo.toml";
        // Preparing
        // Test
        let package_link = open_link(folder);
        let result = package_link.cargo_integrity_check();
        assert!(result);
    }

    #[test]
    fn output_cargo_checker_07_cargo_clean() {
        // Logs
        initialize_log_for_test();
        // Setting
        let folder =
            "tests/output_cargo_checker/output_cargo_checker_07_cargo_clean/project_b/Cargo.toml";
        // Preparing
        // Test
        let package_link = open_link(folder);
        let result = package_link.cargo_clean();
        assert!(result);
    }
}
