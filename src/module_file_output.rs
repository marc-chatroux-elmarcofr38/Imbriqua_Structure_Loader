use anyhow::Result;
use std::path::Path;
use chrono::Local;
use std::fs::{create_dir, read_dir, remove_dir};

use log::{debug, info, error};

#[derive(Clone, Debug)]
pub struct FileEnv {
    pub input_folder : String,
    pub output_folder : String,
}

pub fn get_folders() -> FileEnv {
    /*
        Task function for definition on input folder and output folder
    */

    // Load folders
    let file_env = sub_get_folders();

    // Exit if error
    if file_env.is_err() {
        error!("{}", file_env.err().unwrap());
        error!("Panic : Error during the initialisation of input and output folders");
        panic!("Error during the initialisation of input and output folders");
    }

    file_env.ok().unwrap()
}

fn sub_get_folders() -> Result<FileEnv> {
    /*
        Process function for definition on input folder and output folder
    */
    
    // Check input main folder
    let input_folder = String::from("metamodel_file/");
    let _ = Path::new(input_folder.as_str()).read_dir()?;

    // Check output main folder
    let output_folder = String::from("output_file/");
    let _ = Path::new(output_folder.as_str()).read_dir()?;

    // Get time identifing string
    let time_string = Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();

    // Create sub output_folder
    let output_subfolder = output_folder + time_string.as_str();
    create_dir(&output_subfolder)?;
    info!("folder \"{}\" created", output_subfolder);

    // Check suboutput folder
    let _ = Path::new(output_subfolder.as_str()).read_dir()?;

    // Create file_env
    Ok(FileEnv{
        input_folder : input_folder,
        output_folder : output_subfolder,
    })
}

pub fn get_item_list(file_env : &FileEnv) -> Vec<(String, String)> {
    /*
        Task function for iteration of input files and output files
    */

    // Load folders
    let item_list = sub_get_item_list(file_env);

    // Exit if error
    if item_list.is_err() {
        error!("{}", item_list.err().unwrap());
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
    let iter_input = read_dir(&file_env.input_folder)?;
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
            let output_file = file_env.output_folder.clone() + file.file_name().to_str().unwrap();
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
    let read_result = Path::new(file_env.output_folder.as_str()).read_dir();

    // Remove if empty
    match read_result {
        Ok(mut result) => {
            if result.next().is_none() {
                match remove_dir(&file_env.output_folder) {
                    Ok(_) => {
                        info!("folder \"{}\" deleted (because is empty)", &file_env.output_folder);
                    },
                    Err(error) => {
                        error!("Error during removing of \"{}\" (empty folder) : {}", &file_env.output_folder, error)
                    },
                }
            }            
        },
        Err(error) => {
            error!("Can't evaluate the necessity of removing \"{}\" : {}", &file_env.output_folder, error)
        },
    }
}