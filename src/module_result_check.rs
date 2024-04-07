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
use std::fs::{create_dir, read_to_string, remove_dir, ReadDir};
use log::{error, info, trace};

pub fn move_result(loader_result_file_path : &str, package_result_package_path : &str) {
    //!
    //! AAHH
    //! 

    info!("loader_result_file_path : {}", loader_result_file_path);
    info!("package_result_package_path : {}", package_result_package_path);

    check_file_exist(loader_result_file_path);
    check_folder_exist(package_result_package_path);

}

pub fn check_result(package_result_package_path : &str) {
    //!
    //! AAHH
    //! 

    info!("package_result_package_path : {}", package_result_package_path);
    
}



// #############################################################
//
//    TOOLS FUNCTIONS
//
// #############################################################

fn check_folder_exist(file_path_str : &str) -> () {
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

fn check_read_path(file_path_str : &str) -> ReadDir{
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

fn check_file_exist(file_path_str : &str) -> () {
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
            trace!("CheckFile : Folder \"{}\" exist", &file_path_str);
        },
        false => {
            error!("ERROR_FILE03 - A file don't exist - \"{}\"", &file_path_str);
            panic!("PANIC_FILE03 - A file don't exist - \"{}\"", &file_path_str);
        },
    };
}

fn check_read_file(file_path_str : &str) -> String {
    /*
        Check if the file is readable

        Input :
         - file_path_str (&str) : file path to check

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

fn check_remove_dir(file_path_str : &str) -> bool {
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
        if !check_read_path(file_path_str).next().is_none() {
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
