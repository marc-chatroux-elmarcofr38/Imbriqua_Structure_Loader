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
//! With __copy_result__, this module provide copying file and folder with availibility check.
//! 
//! With __check_result__, this module provite __cargo__ command call (test, doc and other).
//! 
//! ## Minimal usecase
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
//!         // script part generating file in relative path "../Project_B/src/lib.rs"
//!     }
//!     
//!     module_output_checker::check_result("../Project_B/src/lib.rs");
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
//! 
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
//! 
//! 

use std::path::Path;
use std::fs::copy;
use log::{error, debug, info};

pub fn copy_result(loader_result_file_path : &str, relative_path_result_package : &str) {
    //! Copy library folder into a define folder (checking environment)
    //! 

    match Path::new(&loader_result_file_path).exists() {
        true => {
            info!("CheckFile : File \"{}\" exist", &loader_result_file_path);
        },
        false => {
            error!("ERROR_OUT01 - A file don't exist - \"{}\"", &loader_result_file_path);
        },
    };

    match Path::new(&relative_path_result_package).exists() {
        true => {
            info!("CheckFile : Folder \"{}\" exist", &relative_path_result_package);
        },
        false => {
            error!("ERROR_FILE03 - A folder don't exist - \"{}\"", &relative_path_result_package);
        },
    };

    let relative_path_result_package = String::from(relative_path_result_package) + "src/";
    let relative_path_result_package = relative_path_result_package.as_str();

    match Path::new(&relative_path_result_package).exists() {
        true => {
            info!("CheckFile : Folder \"{}\" exist", &relative_path_result_package);
        },
        false => {
            error!("ERROR_FILE03 - A folder don't exist - \"{}\"", &relative_path_result_package);
        },
    };

    let file_result_package_path = String::from(relative_path_result_package) + "lib.rs";
    let file_result_package_path = file_result_package_path.as_str();

    match Path::new(&file_result_package_path).exists() {
        true => {
            info!("CheckFile : File \"{}\" exist", &file_result_package_path);
        },
        false => {
            error!("ERROR_FILE03 - A file don't exist - \"{}\"", &file_result_package_path);
        },
    };

    match copy(loader_result_file_path, file_result_package_path) {
        Ok(_) => {
            info!("Copy OK");
        },
        Err(error) => {
            error!("error during copying : {}", error);
        },
    };
}

pub fn check_result(relative_path_result_package : &str) {
    //! Checking cargo package of a gived folder
    //! 
    //! 
    //! 
    use std::process::Command;
    
    let mut cargo_1 = Command::new("cargo");
    let _ = cargo_1.arg("test")
                   .arg(format!("--manifest-path={}Cargo.toml", relative_path_result_package))
                   .arg("--all-features")
                   .arg("--no-run")
                   .arg("--lib")
                   .output().expect("process failed to execute");
                
    debug!(
        "\n\ncargo test : succes {}\n\nSTDOUT 1 : \n\n{}\n\nSTDERR 1 : \n\n{}",
        cargo_1.output().unwrap().status.success(),
        std::str::from_utf8(&cargo_1.output().unwrap().stdout).unwrap(),
        std::str::from_utf8(&cargo_1.output().unwrap().stderr).unwrap()
    );
    
    let mut cargo_2 = Command::new("cargo");
    let _ = cargo_2.arg("doc")
                   .arg(format!("--manifest-path={}Cargo.toml", relative_path_result_package))
                   .arg("--no-deps")
                   .output().expect("process failed to execute");
                
    debug!(
        "\n\ncargo test : succes {}\n\nSTDOUT 2 : \n\n{}\n\nSTDERR 2 : \n\n{}",
        cargo_2.output().unwrap().status.success(),
        std::str::from_utf8(&cargo_2.output().unwrap().stdout).unwrap(),
        std::str::from_utf8(&cargo_2.output().unwrap().stderr).unwrap()
    );
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