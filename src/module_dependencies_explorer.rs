extern crate minidom;

use std::path::Path;
use std::collections::HashMap;
use std::fs::{create_dir, read_to_string, remove_dir};
use chrono::Local;
use anyhow::Result;
use log::{trace, info, error};
use minidom::Element;

#[derive(Clone, Debug)]
pub struct FileEnv {
    pub input_folder : String,
    output_folder : String,
    pub output_subfolder : String,
}

pub fn get_new_file_env() -> FileEnv{
    FileEnv::new()
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
            // dependencies : Vec::new(),
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
            trace!("Output subfolder created \"{}\"", &file_path_str);
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
            trace!("Folder is readable \"{}\"", file_path_str);
        }
        Err(err_object) => {
            error!("ERROR_FILE02 - \"{}\" : {}", &file_path_str, err_object);
            panic!("{} \"{}\" : {}", &error_str, &file_path_str, err_object);
        }
    }
}

fn file_exist_check(file_path_str : &str, error_str : &str) -> () {
    /*
        Check if the file exist
    */

    // Exit if the folder exist
    match Path::new(&file_path_str).exists() {
        true => {
            trace!("Folder \"{}\" exist", &file_path_str);
        },
        false => {
            error!("ERROR_FILE04 - \"{}\"", &file_path_str);
            panic!("{} \"{}\"", &error_str, &file_path_str);
        },
    };
}

fn file_read_check(file_path_str : &str, error_str : &str) {
    /*
        Check if the file is readable
    */

    // Check if the file is readable 
    match read_to_string(file_path_str) {
        Ok(_) => {
            trace!("File is readable \"{}\"", file_path_str);
        }
        Err(err_object) => {
            error!("ERROR_FILE05 - \"{}\" : {}", &file_path_str, err_object);
            panic!("{} \"{}\" : {}", &error_str, &file_path_str, err_object);
        }
    }
}
















struct LoadingPackage {
    filename : String,
    id : String,
    object : Element,
}

pub struct LoadingTracker {
    file_env : FileEnv,
    loaded_package : HashMap<String, LoadingPackage>,
}

impl LoadingTracker {
    pub fn new() -> Self {

        // Create instance
        let result = LoadingTracker {
            file_env : FileEnv::new(),
            loaded_package : HashMap::new(),
        };

        // Return result
        result
    }

    pub fn load_dependencies(&mut self, main_file : &str, main_package : &str) {
    
        /*/
        let (input_file, output_file) : (String, String) = package_env;
    
        info!("Starting of loading input file \"{}\" to file \"{}\"", &input_file, &output_file);
    
        let sub_run_result = sub_run();
    
        if sub_run_result.is_err() {
            error!("{}", sub_run_result.err().unwrap());
            error!("Panic : Error during loading of a input file");
            panic!("Error during loading of a input file");
        }
    
        info!("End of loading input file \"{}\" to file \"{}\"", &input_file, &output_file);
        */
        let _a = main_file;
        let _b = main_package;
    }

    pub fn close(&self) {
        self.file_env.delete_if_empty();
    }
}
