use anyhow::Result;
use std::path::Path;
use chrono::Local;
use std::fs::{create_dir, read_dir, remove_dir};

use log::{debug, info, error};

#[derive(Clone, Debug)]
pub struct FileEnv {
    pub input_folder : String,
    pub output_subfolder : String,
}

pub fn get_folders() -> FileEnv {
    /*
        Task function for definition of input folder and output folder
    */

    let input_folder = match get_input_folder() {
        Ok(result) => {
            result
        }
        Err(_) => {
            panic!("PANIC_FILE01 - Input main folder isn't readable");
        }
    };

    let output_folder = match get_output_folder() {
        Ok(result) => {
            result
        }
        Err(_) => {
            panic!("PANIC_FILE02 - Output main folder isn't readable");
        }
    };

    let output_subfolder = match create_output_subfolder(output_folder) {
        Ok(result) => {
            result
        }
        Err(_) => {
            panic!("PANIC_FILE03 - Error during creation and controle of output subfolder");
        }
    };

    // Create file_env
    FileEnv{
        input_folder : input_folder,
        output_subfolder : output_subfolder,
    }
}

fn get_input_folder() -> Result<String> {
    /*
        Process function for definition on input folder
    */

    // Check input main folder
    let str_input_folder : &str = "metamodel_file/";

    match Path::new(str_input_folder).read_dir() {
        Ok(_) => {
            debug!("Input main folder is readable \"{}\"", str_input_folder);
            Ok(String::from(str_input_folder))
        }
        Err(err_object) => {
            error!("ERROR_FILE01 - Input main folder isn't readable \"{}\" : {}", str_input_folder, err_object);
            Err(anyhow::Error::new(err_object))
        }
    }
}

fn get_output_folder() -> Result<String> {
    /*
        Process function for definition of output folder
    */

    // Check output main folder
    let str_output_folder : &str = "output_file/";
    
    match Path::new(str_output_folder).read_dir() {
        Ok(_) => {
            debug!("Output main folder is readable \"{}\"", str_output_folder);
            Ok(String::from(str_output_folder))
        }
        Err(err_object) => {
            error!("ERROR_FILE02 - Output main folder isn't readable \"{}\" : {}", str_output_folder, err_object);
            Err(anyhow::Error::new(err_object))
        }
    }
}

fn create_output_subfolder(output_folder : String) -> Result<String> {
    /*
        Process function for definition and creation of output folder
    */

    // Get time identifing string
    let time_string : String = Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();

    // Create sub output_folder
    let output_subfolder = output_folder + time_string.as_str();
    match create_dir(&output_subfolder) {
        Ok(_) => {
            debug!("Output subfolder created \"{}\"", output_subfolder.as_str());
        }
        Err(err_object) => {
            error!("ERROR_FILE03 - Output subfolder uncreatable \"{}\" : {}", output_subfolder.as_str(), err_object);
            return Err(anyhow::Error::new(err_object));
        }
    };

    // Check suboutput folder
    match Path::new(output_subfolder.as_str()).read_dir() {
        Ok(_) => {
            debug!("Output subfolder is readable \"{}\"", output_subfolder.as_str());
        }
        Err(err_object) => {
            error!("ERROR_FILE04 - Output subfolder isn't readable \"{}\" : {}", output_subfolder.as_str(), err_object);
            return Err(anyhow::Error::new(err_object));
        }
    };

    Ok(output_subfolder)
}

pub fn get_item_list(file_env : &FileEnv) -> Vec<(String, String)> {
    /*
        Task function for iteration of input files and output files
    */

    // Load folders
    let item_list = sub_get_item_list(file_env);

    // Exit if error
    if item_list.is_err() {
        error!("Panic : Error during the get of item list");
        panic!("Error during the get of item list");
    }

    item_list.ok().unwrap()
}

fn sub_get_item_list(file_env : &FileEnv) -> Result<Vec<(String, String)>> {
    /*
        Process function for iteration of input files and output files
    */

    // Result object
    let mut result: Vec<(String, String)> = Vec::new();
    
    // Paths in input folder, sorted
    let iter_input = match read_dir(&file_env.input_folder) {
        Ok(result) => {
            debug!("Input main folder readed \"{}\"", &file_env.input_folder);
            result
        }
        Err(err_object) => {
            error!("Input main folder unreadable \"{}\" : {}", &file_env.input_folder, err_object);
            return Err(anyhow::Error::new(err_object));
        }
    };
    let mut iter_input : Vec<_> = iter_input.map(|r| r.unwrap()).collect();
    iter_input.sort_by_key(|dir| dir.path());

    // Explore input folder
    for file in iter_input {
        if !file.file_type().unwrap().is_file() {
            // Don't treat no file path
            debug!("Path \"{}\" is unused because is not a file", file.path().display());
        }
        else {
            debug!("Use path \"{}\"", file.path().display());
            let input_file = String::from(file.path().to_str().unwrap());
            let output_file = file_env.output_subfolder.clone() + file.file_name().to_str().unwrap();
            result.push((input_file, output_file));
        }
    };

    Ok(result)
}

pub fn delete_empty_folders (file_env : FileEnv) {
    /*
        Process function for removing output folder if no result
    */

    // Get path
    let read_result = Path::new(file_env.output_subfolder.as_str()).read_dir();

    // Remove if empty
    match read_result {
        Ok(mut result) => {
            if result.next().is_none() {
                match remove_dir(&file_env.output_subfolder) {
                    Ok(_) => {
                        info!("folder \"{}\" deleted (because is empty)", &file_env.output_subfolder);
                    },
                    Err(error) => {
                        error!("Error during removing of \"{}\" (empty folder) : {}", &file_env.output_subfolder, error)
                    },
                }
            }            
        },
        Err(error) => {
            error!("Can't evaluate the necessity of removing \"{}\" : {}", &file_env.output_subfolder, error)
        },
    }
}
