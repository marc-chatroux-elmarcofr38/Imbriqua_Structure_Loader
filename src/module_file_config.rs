use std::path::Path;
use chrono::Local;
use std::fs::{create_dir, read_dir, remove_dir};

use log::{trace, debug, info, error};

#[derive(Clone, Debug)]
pub struct FileEnv {
    pub input_folder : String,
    output_folder : String,
    pub output_subfolder : String,
}

impl FileEnv {
    pub fn new() -> Self {

        //Set input folder path and output subfolder path
        let str_input_folder : String = String::from("metamodel_file/");
        let str_output_folder : String = String::from("output_file/");
        let time_string : String = Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();

        // Create instance
        let result = FileEnv {
            input_folder : str_input_folder.clone(),
            output_folder : str_output_folder.clone(),
            output_subfolder : str_output_folder.clone() + time_string.as_str(),
        };
        
        // Checking instance
        path_create_dir(&result.input_folder     , "PANIC_FILE01 - Input main folder can't be created");
        path_read_check(&result.input_folder     , "PANIC_FILE02 - Input main folder isn't readable");
        path_create_dir(&result.output_folder    , "PANIC_FILE03 - Output main folder can't be created");
        path_read_check(&result.output_folder    , "PANIC_FILE04 - Output main folder isn't readable");
        path_create_dir(&result.output_subfolder , "PANIC_FILE05 - Output subfolder can't be created");
        path_read_check(&result.output_subfolder , "PANIC_FILE06 - Output subfolder isn't readable");

        // Return result
        result
    }

    pub fn delete_if_empty (&self) {
        /*
            Process function for removing output folder if no result
        */
    
        // Checking instance
        path_read_check(&self.output_subfolder , "PANIC_FILE06 - Output subfolder isn't readable");

        // Remove if empty
        if !Path::new(self.output_subfolder.as_str()).read_dir().unwrap().next().is_none() {
            trace!("Output subfolder isn't empty \"{}\"", self.output_subfolder.as_str());
        } else {
            trace!("Output subfolder is empty \"{}\"", self.output_subfolder.as_str());
            return
        };

        match remove_dir(&self.output_subfolder) {
            Ok(_) => {
                info!("folder \"{}\" deleted (because is empty)", self.output_subfolder);
            },
            Err(error) => {
                error!("ERROR_FILE03 - Error during removing of \"{}\" (empty folder) : {}", self.output_subfolder, error)
            },
        }
    }

    pub fn get_item_list(&self) -> Vec<(String, String)> {
        /*
            Process function for iteration of input files and output files
        */
    
        // Result object
        let mut result: Vec<(String, String)> = Vec::new();

        // Check if path is readable
        path_read_check(&self.input_folder, "PANIC_FILE02 - Input main folder isn't readable");

        // Paths in input folder, sorted
        let iter_input = read_dir(&self.input_folder).unwrap();
        let mut iter_input : Vec<_> = iter_input.map(|r| r.unwrap()).collect();
        iter_input.sort_by_key(|dir| dir.path());

        // Explore input folder
        for file in iter_input {
            if !file.file_type().unwrap().is_file() {
                // Don't treat no file path
                trace!("Path \"{}\" is unused because is not a file", file.path().display());
            }
            else {
                trace!("Use path \"{}\"", file.path().display());
                let input_file = String::from(file.path().to_str().unwrap());
                let output_file = String::from(&self.output_subfolder) + file.file_name().to_str().unwrap();
                result.push((input_file, output_file));
            }
        };

        // Return result
        result
    }
}

fn path_create_dir(file_path_str : &str, error_str : &str) -> () {
    /*
        Check if the folder exist
        Else, try to create it
    */

    // Exit if the folder exist
    match Path::new(&file_path_str).exists() {
        false => {
            trace!("Folder \"{}\" don't exist", &file_path_str);
        },
        true => {
            trace!("Folder \"{}\" exist", &file_path_str);
            return ();
        },
    };

    // Create the dir
    match create_dir(&file_path_str) {
        Ok(_) => {
            debug!("Output subfolder created \"{}\"", &file_path_str);
        }
        Err(err_object) => {
            error!("ERROR_FILE01 - \"{}\" : {}", &file_path_str, err_object);
            panic!("{} \"{}\" : {}", &error_str, &file_path_str, err_object);
        }
    };
}

fn path_read_check(file_path_str : &str, error_str : &str) {
    /*
        Check if the folder is readable
    */

    // Check if the folder is readable
    match Path::new(file_path_str).read_dir() {
        Ok(_) => {
            debug!("Folder is readable \"{}\"", file_path_str);
        }
        Err(err_object) => {
            error!("ERROR_FILE02 - \"{}\" : {}", &file_path_str, err_object);
            panic!("{} \"{}\" : {}", &error_str, &file_path_str, err_object);
        }
    }
}
