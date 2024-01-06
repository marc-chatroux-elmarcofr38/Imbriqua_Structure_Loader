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

pub fn get_folders() -> Result<FileEnv> {
    
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

pub fn get_item_list(file_env : &FileEnv) -> Result<Vec<String>> {

    let mut result: Vec<String> = Vec::new();
    let iter_input = read_dir(&file_env.input_folder)?;

    // Explore input folder
    for file in iter_input {
        if file.as_ref().is_err() {
            // Don't treat error
            error!("Error in a path of \"{}\"", file_env.input_folder);
        }
        else {
            let file = file.unwrap();
            if !file.file_type().unwrap().is_file() {
                // Don't treat no file path
                debug!("Path \"{}\" is unused because is not a file", file.path().display());
            }
            else {
                debug!("Use path \"{}\"", file.path().display());
                result.push(String::from(format!("{:?}", file.path().as_path())));
            }
        }
    };

    Ok(result)
}



pub fn delete_empty_folders (file_env : FileEnv) {
    let read_result = Path::new(file_env.output_folder.as_str()).read_dir();

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