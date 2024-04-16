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
#![doc = include_str!("../doc/module_file_manager.md")]

// Package section

// Dependencies section
extern crate minidom;
use std::path::Path;
use std::fs::{create_dir, read_to_string, remove_dir, ReadDir, File};
use log::{error, trace};
use minidom::Element;

pub fn check_folder_exist(file_path_str : &str) -> () {
    /*
        Make sure that a folder exist
        Check if the folder exist, else, try to create it

        Input :
         - file_path_str (&str) : dir path to create ifdon't exist

        Error :
         - jumping error of std::fs::create_dir
    */

    // Exit if the folder exist
    match Path::new(&file_path_str).exists() {
        false => {
            trace!("CheckFile : Folder \"{}\" don't exist", &file_path_str);
        },
        true => {
            trace!("CheckFile : Folder \"{}\" exist", &file_path_str);
            return ();
        },
    };

    // Create the dir
    match create_dir(&file_path_str) {
        Ok(_) => {
            trace!("CheckFile : Output subfolder created \"{}\"", &file_path_str);
        }
        Err(err_object) => {
            error!("ERROR_FILE01 - A folder can't be created - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE01 - A folder can't be created - \"{}\" : {}", &file_path_str, err_object);
        }
    };
}

pub fn check_read_folder_and_return(file_path_str : &str) -> ReadDir{
    /*
        Check if a folder is readable

        Input :
         - file_path_str (&str) : dir path to check

        Error :
         - jumping error of std::path::Path::read_dir
    */

    // Check if the folder exist
    check_folder_exist(file_path_str);

    // Check if the folder is readable
    match Path::new(file_path_str).read_dir() {
        Ok(result_object) => {
            trace!("CheckFile : Folder is readable \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE02 - A folder isn't readable - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE02 - A folder isn't readable - \"{}\" : {}", &file_path_str, err_object);
        }
    }
}

pub fn check_file_exist(file_path_str : &str) -> () {
    /*
        Check if the file exist

        Input :
         - file_path_str (&str) : file path to check

        Error :
         - none
    */

    // Exit if the folder exist
    match Path::new(&file_path_str).exists() {
        true => {
            trace!("CheckFile : File \"{}\" exist", &file_path_str);
        },
        false => {
            error!("ERROR_FILE03 - A file don't exist - \"{}\"", &file_path_str);
            panic!("PANIC_FILE03 - A file don't exist - \"{}\"", &file_path_str);
        },
    };
}

pub fn check_read_file_and_return(file_path_str : &str) -> String {
    /*
        Check if the file is readable

        Input :
         - file_path_str (&str) : file path to check

        Output :
         - return file content

        Error :
         - jumping error of std::fs::read_to_string
    */

    // Check if the file exist
    check_file_exist(file_path_str);

    // Check if the file is readable 
    match read_to_string(file_path_str) {
        Ok(result_object) => {
            trace!("CheckFile : File is readable \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE04 - A file isn't readable - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE04 - A file isn't readable - \"{}\" : {}", &file_path_str, err_object);
        }
    }
}

pub fn check_remove_dir(file_path_str : &str) -> bool {
    /*
        Check if the folder is deletable (empty)

        Input :
         - file_path_str (&str) : file path to delete

        Output :
         - true if removed, else false

        Error :
         - jumping error of std::fs::remove_dir
    */

        // Exit if not empty
        if !check_read_folder_and_return(file_path_str).next().is_none() {
            trace!("CheckFile : Output subfolder isn't empty \"{}\"", file_path_str);
            return false
        } else {
            trace!("CheckFile : Output subfolder is empty \"{}\"", file_path_str);
        };

        match remove_dir(file_path_str) {
            Ok(_) => {
                return true
            },
            Err(error) => {
                error!("ERROR_FILE50 - Error during removing of a empty folder - \"{}\" : {}", file_path_str, error);
                return false
            },
        }
}

pub fn get_package_from_path(file_path_str : &str, package_id : &str) -> Element {
    /*
        Return the minidom element stored in the input file path

        Input :
         - file_path_str (&str) : file path to read

        Error :
         - jump error of check_read_file
         - jump error of Element parsing
    */
    
    // Check if the file is readable
    let file = check_read_file_and_return(file_path_str);

    // Parsing file content to Element object class
    let element_file : Element = match file.parse() {
        Ok(result_object) => {
            trace!("FileEnv : Reading Element from \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE05 - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE05 - A file isn't parsable - \"{}\" : {}", &file_path_str, err_object);}
    };

    // Find "package_id" child
    for child in element_file.children() {
        if child.is("Package", "http://schema.omg.org/spec/MOF/2.0/cmof.xml") {
            if child.attr("xmi:id") == Some(package_id) {
                return child.clone()
            }
        }
    };

    error!("ERROR_FILE06 - file name = \"{}\", package name = \"{}\"", &file_path_str, package_id);
    panic!("PANIC_FILE06 - CMOF file don't contain the needed package - file name = \"{}\", package name = \"{}\"", &file_path_str, package_id);
}

pub fn create_file(file_path_str : &str) -> File {
    let file = File::create(file_path_str).unwrap();
    file
}
