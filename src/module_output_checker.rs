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

//! Tools for cargo checking of a library folder
//! 
//! # How to use
//! 
//! Allowing to checking (with cargo) a Rust library file or a Rust executable file, previously genereted.
//! 
//! With __purge_folder__, this module provide purging folder (as example, before copying content).
//! 
//! With __copy_result__, this module provide copying file and folder with availibility check.
//! 
//! With __check_result__, this module provite __cargo__ command call (test, doc and other).
//! 
//! ## Minimal usecase
//! 
//! * __/Project_A/main.rs__
//! 
//! ```rust
//! # fn main() {
//! mod module_output_checker;
//! 
//! fn main() {
//! 
//!     fn generate_code () {
//!         // script part generating file in relative path "../Project_B/src/lib.rs"
//!     }
//!     
//!     module_output_checker::check_result("../Project_B/");
//! }
//! # }
//! # fn main() {}
//! ```
//! 
//! * Bash cargo equivalent
//! 
//! ```bash
//! $ cargo test --manifest-path=".../Project_B/" --all-features --no-run --lib
//! $ cargo doc --manifest-path=".../Project_B/" --no-deps
//! ```
//! 
//! * File tree
//! 
//! ```text
//! .
//! ├── Project_A/ (executable package, like Imbriqua_Structure_Loader)
//! │   ├── README.md
//! │   ├── Cargo.toml
//! │   ├── Cargo.lock
//! │   ├── src/
//! │   │   ├── main.rs
//! │   │   ├── module_output_checker.rs
//! │   │   └── ...
//! │   └── ...
//! │
//! ├── Project_B/ (library package to check, like Imbriqua_Structure_Result)
//! │   ├── README.md
//! │   ├── Cargo.toml
//! │   ├── Cargo.lock
//! │   ├── src/
//! │   │   └── lib.rs
//! │   └── ...
//! │
//! └── ...
//! 
//! PS : Of course, you can have a similar folder tree for executable package check
//! ```
//! 
//! 
//! ## Optimal usecase
//! 
//! * __/Project_A/main.rs__
//! 
//! ```
//! # fn main() {
//! mod module_output_checker;
//! 
//! fn main() {
//! 
//!     fn generate_code () {
//!         // script part generating file and complex folder
//!         // in relative path "../Output_file/{time_name}/src"
//!     }
//!     
//!     let output_path = format!("../Output_folder/{}", time_name);
//! 
//!     module_output_checker::purge_folder("../Project_B/src");
//!     module_output_checker::copy_result(output_path.as_str(), "../Project_B/src");
//!     module_output_checker::check_result("../Project_B/");
//! }
//! # }
//! # fn main() {}
//! ```
//! 
//! * Bash cargo equivalent
//! 
//! ```bash
//! $ cargo test --manifest-path=".../Project_B/" --all-features --no-run --lib
//! $ cargo doc --manifest-path=".../Project_B/" --no-deps
//! ```
//! 
//! * File tree
//! 
//! ```text
//! .
//! ├── Imbriqua_Structure_Loader/ (executable package)
//! │   ├── README.md
//! │   ├── Cargo.toml
//! │   ├── Cargo.lock
//! │   ├── src/
//! │   │   ├── main.rs
//! │   │   ├── module_output_checker.rs
//! │   │   └── ...
//! │   └── ...
//! │
//! ├── Imbriqua_Structure_Result/ (library package or executable package)
//! │   ├── README.md
//! │   ├── Cargo.toml
//! │   ├── Cargo.lock
//! │   ├── src/
//! │   │   ├── lib.rs
//! │   │   └── lib_folder/ 
//! │   │       └── ...
//! │   └── ...
//! │
//! ├── Output_folder/
//! │   ├── 2024-04-13_12h36m50/
//! │   │   ├── lib.rs
//! │   │   └── lib_folder/
//! │   │       └── ...
//! │   ├── 2024-04-12_08h47m01/
//! │   │   ├── lib.rs
//! │   │   └── lib_folder/
//! │   │       └── ...
//! │   └── ...
//! │
//! └── ...
//!
//! PS : Of course, you can have a similar folder tree for executable package check
//! ```
//! # Panic and failure
//! 
//! * PANIC_OUT01 - The folder don't exist during purge
//!     * Context : __module_output_checker.rs/purge_folder()__
//!     * Info : Can't find provided folder
//!     * Cause : see details in logs file to get :
//!         * Value of folder path
//!
//! * PANIC_OUT02 - The folder isn't readable during purge
//!     * Context : __module_output_checker.rs/purge_folder()__
//!     * Info : Can't read provided folder
//!     * Cause : see details in logs file to get :
//!         * Value of folder path
//!         * error informations of __std::path::Path::read_dir()__
//!
//! * PANIC_OUT03 - The 'from' folder don't exist (copying)
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Info : Can't find provided folder
//!     * Cause : see details in logs file to get :
//!         * Value of folder path
//!         * error informations of __std::path::Path::exist()__
//! 
//! * PANIC_OUT04 - The 'to' folder don't exist (copying)
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Info : Can't find provided folder
//!     * Cause : see details in logs file to get :
//!         * Value of folder path
//!         * error informations of __std::path::Path::exist()__
//! 
//! * PANIC_OUT05 - The folder isn't readable (copying)
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Info : Can't read provided folder
//!     * Cause : see details in logs file to get :
//!         * Value of folder path
//!         * error informations of __std::path::Path::read_dir()__
//! 
//! * PANIC_OUT06 - Can't copying folder
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Cause : see details in logs file to get :
//!         * Value of folder path
//!         * error informations of __std::fs_extra::copy()__
//! 
//! * PANIC_OUT07 - Can't copying file
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Cause : see details in logs file to get :
//!         * Value of folder path
//!         * error informations of __std::fs::copy()__
//! 
//! * PANIC_OUT08 - Error in ReadDir iterator
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Info : Can't copy provided entry
//!     * Cause : see details in logs file to get :
//!         * error informations of __ReadDir::Iterator__
//! 
//! * PANIC_OUT09 - Error in OsString::to_str()
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Info : Can't copy provided entry
//!     * Cause : see details in documentation of 
//!         * Value of entry (debuging syntax)
//!         * documentation of ReadDir::Iterator
//! 
//! * PANIC_OUT10 - Error in DirEntry::file_type()
//!     * Context : __module_output_checker.rs/copy_result()__
//!     * Info : Can't copy provided entry
//!     * Cause : see details in logs file to get :
//!         * error informations of __DirEntry::file_type__
//! 
//! * WARN_OUT01 - Error in ReadDir iterator
//!     * Context : __module_output_checker.rs/purge_folder()__
//!     * Info : Can't remove provided entry
//!     * Cause : see details in documentation of ReadDir::Iterator
//!
//! * WARN_OUT02 - Error in pathBuf::to_str()
//!     * Context : __module_output_checker.rs/purge_folder()__
//!     * Info : Can't remove provided entry
//!     * Cause : see details in documentation of PathBuf::to_str
//! 
//! * WARN_OUT03 - Error in removing entry
//!     * Context : __module_output_checker.rs/purge_folder()__
//!     * Info : Can't remove provided entry
//!     * Cause : see details in documentation of 
//!         * Value of folder path
//!         * error informations of __std::fs_extra::remove_items()__
//! 

use std::path::Path;
use std::fs;
use fs_extra::dir::{copy, CopyOptions};
use fs_extra::remove_items;
use log::{debug, error, info, warn};
use std::process::Command;

pub fn purge_folder(folder_path : &str) {
    //! Removing all element of a folder

    // Checking if exist
    match Path::new(&folder_path).exists() {
        true => {info!("CheckFile : Folder \"{}\" exist", &folder_path);},
        false => {
            error!("PANIC_OUT01 - The folder don't exist during purge - {}", &folder_path);
            panic!("PANIC_OUT01 - The folder don't exist during purge - {}", &folder_path);
        },
    };

    // Get content
    let items = match fs::read_dir(folder_path) {
        Ok(result) => {result},
        Err(error) => {
            error!("PANIC_OUT02 - The folder isn't readable during purge - {} - {}", &folder_path, error);
            panic!("PANIC_OUT02 - The folder isn't readable during purge - {} - {}", &folder_path, error);
        },
    };

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
                info!("Removing \"{}\"", entry_path);
            },
            Err(error) => {
                error!("WARN_OUT03 - Error in removing entry - {} - {}", &entry_path, error);
            },
        }
    }
}

pub fn copy_result(loader_result_file_path : &str, relative_path_result_package : &str) {
    //! Copy library folder into a define folder (checking environment)

    // Checking if 'loader_result_file_path' exist
    match Path::new(&loader_result_file_path).exists() {
        true => {
            info!("CheckFile : Folder \"{}\" exist", &loader_result_file_path);
        },
        false => {
            error!("PANIC_OUT03 - The 'from' folder don't exist (copying) - \"{}\"", &loader_result_file_path);
            panic!("PANIC_OUT03 - The 'from' folder don't exist (copying) - \"{}\"", &loader_result_file_path);
        },
    };

    // Checking if 'relative_path_result_package' exist
    match Path::new(&relative_path_result_package).exists() {
        true => {
            info!("CheckFile : Folder \"{}\" exist", &relative_path_result_package);
        },
        false => {
            error!("PANIC_OUT04 - The 'to' folder don't exist (copying) - \"{}\"", &relative_path_result_package);
            panic!("PANIC_OUT04 - The 'to' folder don't exist (copying) - \"{}\"", &relative_path_result_package);
        },
    };

    let options = CopyOptions::new(); 

    // Get content
    let items = match fs::read_dir(loader_result_file_path) {
        Ok(result) => {result},
        Err(error) => {
            error!("PANIC_OUT05 - The folder isn't readable (copying) - {} - {}", &loader_result_file_path, error);
            panic!("PANIC_OUT05 - The folder isn't readable (copying) - {} - {}", &loader_result_file_path, error);
        },
    };

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
        let mut fr = String::from(loader_result_file_path);
        fr.push_str(entry_name);
        // To
        let mut go = String::from(relative_path_result_package);
        go.push_str(entry_name);

        if entry_type.is_dir() {
            match copy(&fr, relative_path_result_package, &options) {
                Ok(_) => {
                    info!("Copying folder \"{}\" to \"{}\"", fr, go);
                },
                Err(error) => {
                    error!("PANIC_OUT06 - Can't copying \"{}\" folder to \"{}\" - {}", fr, go, error);
                    panic!("PANIC_OUT06 - Can't copying \"{}\" folder to \"{}\" - {}", fr, go, error);
                },
            }
        } else {
            match fs::copy(&fr, &go) {
                Ok(_) => {
                    info!("Copying file \"{}\" to \"{}\"", fr, go);
                },
                Err(error) => {
                    error!("PANIC_OUT07 - Can't copying \"{}\" file to \"{}\" - {}", fr, go, error);
                    panic!("PANIC_OUT07 - Can't copying \"{}\" file to \"{}\" - {}", fr, go, error);
                },
            }
        }
    };
}

pub fn check_result(relative_path_result_package : &str) {
    //! Checking package of a gived folder, with cargo bash command

    let mut cargo_1 = Command::new("cargo");
    let _ = cargo_1.arg("test")
                   .arg(format!("--manifest-path={}Cargo.toml", relative_path_result_package))
                   .arg("--all-features")
                   .arg("--no-run")
                   .arg("--lib")
                   .output().expect("process failed to execute");

    represent_command_output(&mut cargo_1);
    
    let mut cargo_2 = Command::new("cargo");
    let _ = cargo_2.arg("doc")
                   .arg(format!("--manifest-path={}Cargo.toml", relative_path_result_package))
                   .arg("--no-deps")
                   .output().expect("process failed to execute");
                
    represent_command_output(&mut cargo_2);
}

fn represent_command_output(command : &mut Command) {
    //! Printing command result
                
    let command_output = command.output().unwrap();
    let str_stdout = std::str::from_utf8(&command_output.stdout).unwrap();
    let str_stderr = std::str::from_utf8(&command_output.stderr).unwrap();

    debug!(
        "\n\ncargo test : succes {}\n\nSTDOUT 2 : \n\n{}\n\nSTDERR 2 : \n\n{}",
        command_output.status.success(), str_stdout, str_stderr
    );

    let succes = if command_output.status.success() {"SUCCES"} else {"ERROR"};
    info!("Get {} on {:#?}", succes, &command);
}

#[cfg(test)]
mod copy_result {
    #[test]
    fn file_1() {

    }

    #[test]
    #[should_panic]
    fn file_2() {
        panic!()
    }
}

#[cfg(test)]
mod check_result {
    #[test]
    fn file_1() {

    }

    #[test]
    #[should_panic]
    fn file_2() {
        panic!()
    }
}