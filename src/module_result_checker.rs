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

//! 
//! AAHH
//! 

use std::path::Path;
use std::fs::copy;
use log::{error, debug, info};

pub fn copy_result(loader_result_file_path : &str, package_result_package_path : &str) {
    //!
    //! AAHH
    //! 
    //! 

    match Path::new(&loader_result_file_path).exists() {
        true => {
            info!("CheckFile : File \"{}\" exist", &loader_result_file_path);
        },
        false => {
            error!("ERROR_FILE03 - A file don't exist - \"{}\"", &loader_result_file_path);
        },
    };

    match Path::new(&package_result_package_path).exists() {
        true => {
            info!("CheckFile : Folder \"{}\" exist", &package_result_package_path);
        },
        false => {
            error!("ERROR_FILE03 - A folder don't exist - \"{}\"", &package_result_package_path);
        },
    };

    let package_result_package_path = String::from(package_result_package_path) + "src/";
    let package_result_package_path = package_result_package_path.as_str();

    match Path::new(&package_result_package_path).exists() {
        true => {
            info!("CheckFile : Folder \"{}\" exist", &package_result_package_path);
        },
        false => {
            error!("ERROR_FILE03 - A folder don't exist - \"{}\"", &package_result_package_path);
        },
    };

    let file_result_package_path = String::from(package_result_package_path) + "lib.rs";
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

pub fn check_result(package_result_package_path : &str) {
    //!
    //! AAHH
    //! 
    use std::process::Command;
    
    let mut cargo_2 = Command::new("cargo");
    let _ = cargo_2.arg("test")
                   .arg(format!("--manifest-path={}Cargo.toml", package_result_package_path))
                   .arg("--all-features")
                   .arg("--no-run")
                   .arg("--lib")
                   .arg("--package=imbriqua_structure_result")
                   .output().expect("process failed to execute");
                
    debug!(
        "\n\ncargo test : succes {}\n\nSTDOUT 2 : \n\n{}\n\nSTDERR 2 : \n\n{}",
        cargo_2.output().unwrap().status.success(),
        std::str::from_utf8(&cargo_2.output().unwrap().stdout).unwrap(),
        std::str::from_utf8(&cargo_2.output().unwrap().stderr).unwrap()
    );
}
