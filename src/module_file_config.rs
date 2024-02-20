extern crate minidom;

use std::path::Path;
use chrono::Local;
use std::fs::{create_dir, read_to_string, remove_dir};
use minidom::Element;

use log::{trace, debug, info, error};

#[derive(Clone, Debug)]
pub struct FileEnv {
    pub input_folder : String,
    output_folder : String,
    pub output_subfolder : String,
    dependencies : Vec<(String, String)>,
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
            dependencies : Vec::new(),
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

    pub fn get_package(&mut self, main_file : &str) -> Vec<(String, String)> {
        /*
            Process function for iteration of input files and output files
        */
    
        // If already calculate
        if self.dependencies.is_empty() {
            // Check if path is readable
            path_read_check(&self.input_folder, "PANIC_FILE02 - Input main folder isn't readable");
    
            // Check if the main file exist
            self.add_dependencies_of(main_file);
        };

        // Return result
        debug!("{:?}", self.dependencies);
        self.dependencies.clone()
    }
/*
    fn get_sorted_files(&self) -> Vec<DirEntry> {

        // Paths in input folder, sorted
        let iter_input = read_dir(&self.input_folder).unwrap();
        let mut iter_input : Vec<DirEntry> = iter_input.map(|r| r.unwrap()).collect();
        iter_input.sort_by_key(|dir| dir.path());

        iter_input
    }
*/
    fn add_dependencies_of(&mut self, main_file : &str) {
        /*
            Read main files
            Find dependencies files
            Add it to self.dependencies
        */

        // Check if main file exist and is readable
        let mut file : String = self.input_folder.clone();
        file.push_str(main_file);
        file_exist_check(&file.as_str(), "PANIC_FILE07 - A CMOF dependencies doesn't exist");
        file_read_check(&file.as_str(), "PANIC_FILE08 - A CMOF dependencies isn't readable");

        // Add main file to dependencies
        let mut new = Vec::new();
        new.push((file.clone(), String::from(&self.output_subfolder) + main_file));
        self.dependencies.retain(|x| x != &new[0]);
        self.dependencies.splice(0..0, new);

        // Find dependencies
        let mut dependencies_file = Vec::new();

        debug!("{}", file);
        
        let str_tree = read_to_string(file).unwrap();
        let xml_tree : Element = match str_tree.parse() {
            Ok(result) => {result}
            Err(_) => {panic!("bbbb")}
        };

        for child_1 in xml_tree.children() {
            if child_1.name() == "Package"{
                for child_2 in child_1.children() {
                    if child_2.name() == "packageImport"{
                        
                    }
                }
            }
        }

/*
        if main_file == "DI.cmof" {
            dependencies_file.push("DC.cmof");
        }
        else if main_file == "BPMNDI.cmof" {
            dependencies_file.push("DI.cmof");
            dependencies_file.push("DC.cmof");
            dependencies_file.push("BPMN20.cmof");
        };*/

        // Find dependencies of dependencies
        for file in dependencies_file {
            self.add_dependencies_of(file);
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
