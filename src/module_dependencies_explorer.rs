extern crate minidom;

use std::path::Path;
use std::collections::HashMap;
use std::fs::{create_dir, read_to_string, remove_dir, ReadDir};
use chrono::Local;
use log::{trace, info, error, warn};
use minidom::Element;

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
            // dependencies : Vec::new(),
        };
        
        // Checking instance
        check_folder_exist(&result.input_folder);
        check_read_path(&result.input_folder);
        check_folder_exist(&result.output_folder);
        check_read_path(&result.output_folder);
        check_folder_exist(&result.output_subfolder);
        check_read_path(&result.output_subfolder);

        // Return result
        result
    }

    pub fn delete_if_empty (&self) {
        /*
            Process function for removing output folder if no result
        */
    
        if check_remove_dir(&self.output_subfolder.as_str()) {
                info!("folder \"{}\" deleted (because is empty)", self.output_subfolder);
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

    pub fn load_dependencies_file(&mut self, main_file : &str, main_package : &str) {
        /*
            Load minidom element from a gived package, including dependencies
            Save element in loaded_package
    
            Input :
             - main_file (&str) : file to load in self.input_folder
             - main_package (&str) : package name wanted in main_file
    
            Error :
             - none
        */
        
        // Add empty element entry in loaded_package (prevent circular loading)
        self.add_empty_package(main_file, main_package);
    
        // Generate file path
        let mut file_path = self.file_env.input_folder.clone();
        file_path.push_str(main_file);

        // Load package element
        let package_element = get_package_from_path(file_path.as_str(), main_package);

        // Evaluate dependencies, and load it
        self.add_dependencies(&package_element);
        
        // Add package element in loaded_package
        self.add_package(package_element, main_file, main_package);
    }

    fn add_dependencies(&mut self, element : &Element) {
        /*
        
        */

        for child in element.children() {
            if child.is("packageImport", "") {
                warn!("need to load \"{}\"", child.children().next().unwrap().attr("href").unwrap())
            }
        }
    }

    fn add_empty_package(&mut self, file : &str, package : &str) {
        /*
            Save minimal LoadingPackage object in loaded_package, to prevent circular dependencies loading
    
            Input :
             - file (&str) : file to load in self.input_folder
             - package (&str) : package name wanted in main_file
    
            Error :
             - none
        */

        // Create minimal LoadingPackage
        let package_object = LoadingPackage {
            filename : String::from(file),
            id : String::from(package),
            object : Element::builder("", "").build(),
        };

        // Define hashmap key
        let mut label = String::from(file);
        label.push_str(":");
        label.push_str(package);

        // Save object in hashmap attribute
        self.loaded_package.insert(label, package_object);
    }

    fn add_package(&mut self, element : Element, file : &str, package : &str) {
        /*
            Save complete LoadingPackage object in loaded_package
    
            Input :
             - element (minidom::element::Element) : Element to save
             - file (&str) : file to load in self.input_folder
             - package (&str) : package name wanted in main_file
    
            Error :
             - none
        */

        // Create full LoadingPackage
        let package_object = LoadingPackage {
            filename : String::from(file),
            id : String::from(package),
            object : element,
        };

        // Define hashmap key
        let mut label = String::from(file);
        label.push_str(":");
        label.push_str(package);


        // Save object in hashmap attribute
        self.loaded_package.insert(label, package_object);
    }

    fn is_package_already_loaded(&self, file : &str, package : &str) -> bool {

        // Define hashmap key
        let mut label = String::from(file);
        label.push_str(":");
        label.push_str(package);

        // Check if the key is already used
        self.loaded_package.contains_key(&label)
    }

    pub fn close(&self) {
        self.file_env.delete_if_empty();
    }
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
            trace!("Folder is readable \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE02 - \"{}\" : {}", &file_path_str, err_object);
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
            trace!("Folder \"{}\" exist", &file_path_str);
        },
        false => {
            error!("ERROR_FILE03 - \"{}\" don't exist", &file_path_str);
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
            trace!("File is readable \"{}\"", file_path_str);
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE04 - \"{}\" : {}", &file_path_str, err_object);
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
            trace!("Output subfolder isn't empty \"{}\"", file_path_str);
            return false
        } else {
            trace!("Output subfolder is empty \"{}\"", file_path_str);
        };

        match remove_dir(file_path_str) {
            Ok(_) => {
                return true
            },
            Err(error) => {
                error!("ERROR_FILE10 - Error during removing of a empty folder - \"{}\" : {}", file_path_str, error);
                return false
            },
        }
}

fn get_package_from_path(file_path_str : &str, package_name : &str) -> Element {
    /*
        Return the minidom element stored in the input file path

        Input :
         - file_path_str (&str) : file path to read

        Error :
         - jump error of check_read_file
         - jump error of Element parsing
    */
    
    // Check if the file is readable
    let file = check_read_file(file_path_str);

    // Parsing file content to Element object class
    let element_file : Element = match file.parse() {
        Ok(result_object) => {
            result_object
        }
        Err(err_object) => {
            error!("ERROR_FILE05 - \"{}\" : {}", &file_path_str, err_object);
            panic!("PANIC_FILE05 - A file isn't parsable - \"{}\" : {}", &file_path_str, err_object);}
    };

    // Find "package_name" child
    for child in element_file.children() {
        if child.is("Package", "http://schema.omg.org/spec/MOF/2.0/cmof.xml") {
            if child.attr("name") == Some(package_name) {
                return child.clone()
            }
        }
    };

    error!("ERROR_FILE06 - file name = \"{}\", package name = \"{}\"", &file_path_str, package_name);
    panic!("PANIC_FILE06 - CMOF file don't contain the needed package - file name = \"{}\", package name = \"{}\"", &file_path_str, package_name);
}
