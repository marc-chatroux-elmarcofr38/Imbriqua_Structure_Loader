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
#![doc = include_str!("../doc/module_output_checker.md")]

// Package section
use crate::module_file_manager::{FileManager, PathBuf, Path};
use crate::module_log::*;

// Dependencies section
// use std::fs;
use std::{process::Command, fs::canonicalize};
use fs_extra::remove_items;
// use fs_extra::{dir::copy, dir::CopyOptions, remove_items};


#[derive(Clone, PartialEq, Debug)]
/// Represent a cargo package folder link, used to checking metacode result
pub struct PackageLink {
    /// PathBuf of the Cargo.toml file of the package
    absolute_path : PathBuf,
}

impl PackageLink {
    pub fn from(str_relative_cargo_path : &str) -> Self {
        //! Initialise a cargo package folder link from relative string path
        //!
        //! str_relative_cargo_path (&str) : relative path of the Cargo.toml file to link

        // Make it absolute
        let var_absolute_path = match canonicalize(str_relative_cargo_path) {
            Ok(result) => {
                info!("Can canonicalize {:?} to {:?}", str_relative_cargo_path, result);
                result
            },
            Err(error) => {
                error!("PANIC_OUT01 - Can't canonicalize {:?} - {}", str_relative_cargo_path, error);
                panic!("PANIC_OUT01 - Can't canonicalize {:?} - {}", str_relative_cargo_path, error);
            },
        };

        // Instanciate object
        let result =
            PackageLink {
                absolute_path : var_absolute_path
            };

        // Checking file integrity
        result.cargo_integrity_check();

        // Return
        result
    }

    pub fn get_absolute_cargo_path(&self) -> PathBuf {
        self.absolute_path.clone()
    }

    pub fn get_absolute_root_path(&self) -> PathBuf {
        let mut result = self.absolute_path.clone();
        result.pop();
        result
    }

    pub fn get_absolute_source_path(&self) -> PathBuf {
        let mut result = self.absolute_path.clone();
        result.pop();
        result.push("src");
        result
    }

    pub fn cargo_custom_command(&self, args : Vec<&str>) -> bool {
        //! Running cargo custom command
        //!
        //! args (Vec<&str>) : list of arg to forward to the command
        //!
        //! # Errors
        //!
        //! See module_output_checker documentation page for errors details
        //!
        //! # Examples
        //!
        //! See module_output_checker documentation page for examples

        let mut cargo_1 = Command::new("cargo");
        let _ = cargo_1.args(args)
                       .arg(format!("--manifest-path={}", self.absolute_path.to_string_lossy()))
                       .output().expect("process failed to execute");

        let result_1 = represent_command_output(&mut cargo_1).is_some_and(|x| x == true);
        trace!("Running cargo command : {} : {:?}", if result_1 {"succes"} else {"error"}, cargo_1);

        result_1
    }

    pub fn cargo_full_check(&self) -> bool {
        //! Running cargo full check (check, test, build, doc)
        //!
        //! # Errors
        //!
        //! See module_output_checker documentation page for errors details
        //!
        //! # Examples
        //!
        //! See module_output_checker documentation page for examples

        let result_1 = self.cargo_custom_command(vec!["check", "--all-features", "--lib"]);
        let result_2 = self.cargo_custom_command(vec!["test", "--all-features", "--no-run", "--lib"]);
        let result_3 = self.cargo_custom_command(vec!["build", "--all-features", "--lib"]);
        let result_4 = self.cargo_custom_command(vec!["doc", "--no-deps"]);

        result_1 && result_2 && result_3 && result_4
    }

    pub fn cargo_integrity_check(&self) {
        //! Running cargo locate-project command, allowing to check existence of Cargo.toml file
        //!
        //! # Errors
        //!
        //! See module_output_checker documentation page for errors details
        //!
        //! # Examples
        //!
        //! See module_output_checker documentation page for examples

        match &self.cargo_custom_command(vec!["locate-project"]) {
            true => {
                info!("Cargo.toml exist for {:?}", &self.absolute_path);
            },
            false => {
                error!("PANIC_OUT02 - Can't find cargo.toml {:?}", &self.absolute_path);
                panic!("PANIC_OUT02 - Can't find cargo.toml {:?}", &self.absolute_path);
            },
        };
    }

    pub fn cargo_clean(&self) {
        //! Running cargo clean command, allowing to purge "target" folder
        //!
        //! # Errors
        //!
        //! See module_output_checker documentation page for errors details
        //!
        //! # Examples
        //!
        //! See module_output_checker documentation page for examples

        match &self.cargo_custom_command(vec!["clean"]) {
            true => {
                info!("Cleaning {:?} package", &self.absolute_path);
            },
            false => {
                error!("PANIC_OUT02 - Can't clean {:?}", &self.absolute_path);
                panic!("PANIC_OUT02 - Can't clean {:?}", &self.absolute_path);
            },
        };
    }

    pub fn purge (&self) {
        //! Removing subelement of a "/src" folder
        //!
        //! # Errors
        //!
        //! See module_output_checker documentation page for errors details
        //!
        //! # Examples
        //!
        //! See module_output_checker documentation page for examples

        // Get content
        let source_path = self.get_absolute_source_path();
        let items = source_path.get_folder_content();
        // module_file_manager::check_read_folder_and_return( );

        // Remove each entry
        for entry in items {
            let entry_path = match entry {
                Ok(result) => {
                    result.path()
                },
                Err(_) => {
                    warn!("WARN_OUT_01 - Error in ReadDir iterator");
                    continue;
                },
            };
            let entry_path = entry_path.to_str();

            let entry_path = match entry_path {
                Some(result) => {
                    result
                },
                None => {
                    warn!("WARN_OUT_02 - Error in pathBuf::to_str()");
                    continue;
                },
            };

            let mut vec_entry = Vec::new();
            vec_entry.push(entry_path);

            match remove_items(&vec_entry) {
                Ok(_) => {
                    trace!("purge : Removing \"{}\"", entry_path);
                },
                Err(error) => {
                    error!("WARN_OUT03 - Error in removing entry - {} - {}", &entry_path, error);
                },
            }
        }
    }

    pub fn load_from(&self, from_path : PathBuf) {
        //! Copy all subelement of a source folder in a target folder
        //!
        //! from_path (&str) : source folder
        //! to_path (&str) : target forder
        //!
        //! # Errors
        //!
        //! See module_output_checker documentation page for errors details
        //!
        //! # Examples
        //!
        //! See module_output_checker documentation page for examples

        from_path.copy_folder(&self.get_absolute_source_path());
/*
        // Checking 'from_path'
        let items = from_path.get_folder_content();

        // Checking 'to_path'
        let to_path = self.get_source().get_folder_content();

        let options = CopyOptions::new();

        // Copying each entry
        for entry in items {
            // ReadDir error
             let entry = match entry {
                Ok(result) => {
                    result
                },
                Err(error) => {
                    error!("PANIC_OUT08 - Error in ReadDir iterator - {}", error);
                    panic!("PANIC_OUT08 - Error in ReadDir iterator - {}", error);
                },
            };
            let entry_name = entry.file_name();

            // OsString error
            let entry_name = match entry_name.to_str() {
                Some(result) => {
                    result
                },
                None => {
                    error!("PANIC_OUT09 - Error in OsString::to_str() - {:?}", entry_name);
                    panic!("PANIC_OUT09 - Error in OsString::to_str() - {:?}", entry_name);
                },
            };

            // file_type error
            let entry_type = match entry.file_type() {
                Ok(result) => {
                    result
                },
                Err(error) => {
                    error!("PANIC_OUT10 - Error in DirEntry::file_type() - {}", error);
                    panic!("PANIC_OUT10 - Error in DirEntry::file_type() - {}", error);
                },
            };

            // From
            let mut fr = String::from(from_path);
            fr.push_str(entry_name);
            // To
            let mut go = String::from(to_path);
            go.push_str(entry_name);

            if entry_type.is_dir() {
                match copy(&fr, to_path, &options) {
                    Ok(_) => {
                        info!("copy : copying folder \"{}\" to \"{}\"", fr, go);
                    },
                    Err(error) => {
                        error!("PANIC_OUT06 - Can't copying \"{}\" folder to \"{}\" - {}", fr, go, error);
                        panic!("PANIC_OUT06 - Can't copying \"{}\" folder to \"{}\" - {}", fr, go, error);
                    },
                }
            } else {
                match fs::copy(&fr, &go) {
                    Ok(_) => {
                        info!("copy : copying file \"{}\" to \"{}\"", fr, go);
                    },
                    Err(error) => {
                        error!("PANIC_OUT07 - Can't copying \"{}\" file to \"{}\" - {}", fr, go, error);
                        panic!("PANIC_OUT07 - Can't copying \"{}\" file to \"{}\" - {}", fr, go, error);
                    },
                }
            }
        };*/

    }
}

#[doc(hidden)]
fn represent_command_output(command : &mut Command) -> Option<bool> {
    //! Printing command result, used by __check_result__ function
    //!
    //! # Errors
    //!
    //! See module_output_checker documentation page for errors details
    //!
    //! # Examples
    //!
    //! See module_output_checker documentation page for examples

    let command_output= match command.output() {
        Ok(result) => {
            result
        },
        Err(error) => {
            warn!("WARN_OUT04 - Cound't get output of {:?} - {}", &command, error);
            return None;
        },
    };

    let str_stdout = match std::str::from_utf8(&command_output.stdout) {
        Ok(result) => {
            result
        },
        Err(error) => {
            warn!("WARN_OUT05 - Couldn't get STDOUT of {:?} - {}", &command, error);
            return None;
        },
    };

    let str_stderr = match std::str::from_utf8(&command_output.stderr) {
        Ok(result) => {
            result
        },
        Err(error) => {
            warn!("WARN_OUT06 - Couldn't get STDERR of {:?} - {}", &command, error);
            return None;
        },
    };

    debug!("\nsucces:\n{}\ncommand:\n{:#?}\nstdout:\n{}\nstderr:\n{}", command_output.status.success(), &command, str_stdout, str_stderr);
    Some(command_output.status.success())
}

/*
#[cfg(test)]
mod tests {
    use crate::module_output_checker::purge_folder;
    use crate::module_output_checker::copy_folder;
    use crate::module_output_checker::check_result;
    use crate::module_output_checker::clean_target_result;
    use crate::module_output_checker::cargo_custom_command;

    #[test]
    fn module_out_01_succes_all_in_one() {
        // Copy
        copy_folder("tests/module_out_01_succes_all_in_one/output/", "tests/module_out_01_succes_all_in_one/project_b/src/");
        // Check
        let result = check_result("tests/module_out_01_succes_all_in_one/project_b/");
        assert_eq!(result, true);
        // Build
        let result = cargo_custom_command(vec!["build"], "tests/module_out_01_succes_all_in_one/project_b/");
        assert_eq!(result, true);
        // Clean
        let result = clean_target_result("tests/module_out_01_succes_all_in_one/project_b/");
        assert_eq!(result, true);
        // Purge
        purge_folder("tests/module_out_01_succes_all_in_one/project_b/src/");
    }

    #[test]
    #[should_panic(expected = "PANIC_OUT01")]
    fn module_out_02_panic_out01() {
        //! Folder don't exist
        purge_folder("tests/module_out_02_panic_out01/to/");
    }

    // module_out_03_panic_out02 - Can't generate read/write error on GitHub repositories folder

    #[test]
    #[should_panic(expected = "PANIC_OUT03")]
    fn module_out_04_panic_out03() {
        //! Source folder don't exist
        purge_folder("tests/module_out_04_panic_out03/to/");
        copy_folder("tests/module_out_04_panic_out03/from/", "tests/module_out_04_panic_out03/to/");
    }

    #[test]
    #[should_panic(expected = "PANIC_OUT04")]
    fn module_out_05_panic_out04() {
        //! Target folder don't exist
        copy_folder("tests/module_out_05_panic_out04/from/", "tests/module_out_05_panic_out04/to/");
    }

    // module_out_06_panic_out05 - Can't generate read/write error on GitHub repositories folder

    // module_out_07_panic_out06 - Can't generate read/write error on GitHub repositories folder

    // module_out_08_panic_out07 - Can't generate read/write error on GitHub repositories folder

    // module_out_09_panic_out08 - Can't generate read/write error on GitHub repositories folder

    // module_out_10_panic_out09 - Can't generate read/write error on GitHub repositories folder

    // module_out_11_panic_out10 - Can't generate read/write error on GitHub repositories folder

    #[test]
    fn module_out_11_check_result_return_false() {
        //! This folder can be copied
        let result = check_result("tests/module_out_11_check_result_return_false/project_b/");
        assert_eq!(result, false);
        let result = clean_target_result("tests/module_out_11_check_result_return_false/project_b/");
        assert_eq!(result, true);
    }
}
*/
