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
pub use std::path::Path;
use std::fs::{ReadDir, File};
use fs_extra;
use log::{error, trace};
use minidom::Element;

/// Provide shortcut to filesystem function, with error forwarding and panic control
pub trait FileManager {
    ///  Return the content of the folder (as ReadDir)
    fn get_folder_content(&self) -> ReadDir;

    /// Create the folder if don't exist
    fn create_folder(&self) -> ();

    /// Copy each item of a folder to a other
    fn copy_folder(&self, to : &Self) -> ();

    /// Move each item of a folder to a other
    fn move_folder(&self, to : &Self) -> ();

    /// Delete the folder if it exist
    fn delete_folder(&self, empty_only : bool) -> ();

    /// Check if the file exist, and if it's file, and if it's readable and return it (as File)
    fn get_file(&self) -> File;

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as String)
    fn get_file_content(&self) -> String;

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as Element)
    fn get_file_content_as_element(&self) -> Element;

    /// Create the file if don't exist
    fn create_file(&self) -> ();

    /// Copy a file to a other location
    fn copy_file(&self, to : &Self) -> ();

    /// Move a file to a other location
    fn move_file(&self, to : &Self) -> ();

    /// Delete the file if it exist
    fn delete_file(&self) -> ();
}

impl FileManager for Path {
    ///  Return the content of the folder (as ReadDir)
    fn get_folder_content(&self) -> ReadDir {
        // Get the content
        match self.read_dir() {
            Ok(result_object) => {
                trace!("The 'folder' is readable : {:?}", self);
                result_object
            }
            Err(error) => {
                error!("PANIC_FLM01 - The 'folder' isn't readable : {:?} (err : {})", self, error);
                panic!("PANIC_FLM01 - The 'folder' isn't readable : {:?} (err : {})", self, error);
            }
        }
    }

    /// Create the folder if don't exist
    fn create_folder(&self) -> ()  {
        // Exit if the folder exist
        if self.exists() {
            trace!("Folder {:?} already exist (don't create)", self);
            return;
        }
        // Else, create it
        match fs_extra::dir::create_all(self, false) {
            Ok(_) => {
                trace!("Folder {:?} created", self);
            }
            Err(error) => {
                error!("PANIC_FLM02 - The 'folder' can't be created : {:?} (err : {})", self, error);
                panic!("PANIC_FLM02 - The 'folder' can't be created : {:?} (err : {})", self, error);
            }
        };
    }

    /// Copy each item of a folder to a other
    fn copy_folder(&self, to : &Self) -> () {
        // Setting options
        let mut options_1 = fs_extra::dir::DirOptions::new();
        options_1.depth = 1;
        let options_2 = fs_extra::dir::CopyOptions::new();

        // Get items
        let mut from_paths: Vec<String> = Vec::new();
        match fs_extra::dir::get_dir_content2(self, &options_1) {
            Ok(result) => {
                from_paths.extend(result.files);
                from_paths.extend(result.directories);
            },
            Err(error) => {
                error!("PANIC_FLM03 - The 'folder' can't be copied : {:?} (err_01 : {})", self, error);
                panic!("PANIC_FLM03 - The 'folder' can't be copied : {:?} (err_01 : {})", self, error);
            },
        }
        from_paths.retain(|x| Path::new(&x) != self);

        // Copying file
        match fs_extra::copy_items(&from_paths, to , &options_2) {
            Ok(_) => {
                trace!("File {:?} copied", self);
            }
            Err(error) => {
                error!("PANIC_FLM03 - The 'folder' can't be copied : {:?} (err_02 : {})", self, error);
                panic!("PANIC_FLM03 - The 'folder' can't be copied : {:?} (err_02 : {})", self, error);
            }
        };
    }

    /// Move each item of a folder to a other
    fn move_folder(&self, to : &Self) -> () {
        // Setting options
        let mut options_1 = fs_extra::dir::DirOptions::new();
        options_1.depth = 1;
        let options_2 = fs_extra::dir::CopyOptions::new();

        // Get items
        let mut from_paths: Vec<String> = Vec::new();
        match fs_extra::dir::get_dir_content2(self, &options_1) {
            Ok(result) => {
                from_paths.extend(result.files);
                from_paths.extend(result.directories);
            },
            Err(error) => {
                error!("PANIC_FLM04 - The 'folder' can't be moved : {:?} (err_01 : {})", self, error);
                panic!("PANIC_FLM04 - The 'folder' can't be moved : {:?} (err_01 : {})", self, error);
            },
        }
        from_paths.retain(|x| Path::new(&x) != self);


        // Moving file
        match fs_extra::move_items(&from_paths, to , &options_2) {
            Ok(_) => {
                trace!("File {:?} moved", self);
            }
            Err(error) => {
                error!("PANIC_FLM04 - The 'folder' can't be moved : {:?} (err_02 : {})", self, error);
                panic!("PANIC_FLM04 - The 'folder' can't be moved : {:?} (err_02 : {})", self, error);
            }
        };
    }

    /// Delete the folder if it exist
    fn delete_folder(&self, empty_only : bool) -> () {
        // Exit if not empty AND empty constraint
        if empty_only && self.get_folder_content().next().is_none() {
            trace!("Folder {:?} isn't empty (don't delete)", self);
            return;
        }
        // Create it
        match fs_extra::dir::remove(self) {
            Ok(_) => {
                trace!("Folder {:?} deleted", self);
            }
            Err(error) => {
                error!("PANIC_FLM05 - The 'folder' can't be deleted : {:?} (err : {})", self, error);
                panic!("PANIC_FLM05 - The 'folder' can't be deleted : {:?} (err : {})", self, error);
            }
        };
    }

    /// Check if the file exist, and if it's file, and if it's readable and return it (as File)
    fn get_file(&self) -> File {
        // Get the content
        match File::open(self) {
            Ok(result_object) => {
                trace!("The 'file' is readable : {:?}", self);
                result_object
            }
            Err(error) => {
                error!("PANIC_FLM06 - The 'file' isn't readable (as File) : {:?} (err : {})", self, error);
                panic!("PANIC_FLM06 - The 'file' isn't readable (as File) : {:?} (err : {})", self, error);
            }
        }
    }

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as String)
    fn get_file_content(&self) -> String {
        // Get the content
        match fs_extra::file::read_to_string(self) {
            Ok(result_object) => {
                trace!("The 'file' is readable : {:?}", self);
                result_object
            }
            Err(error) => {
                error!("PANIC_FLM07 - The 'file' isn't readable (as String) : {:?} (err : {})", self, error);
                panic!("PANIC_FLM07 - The 'file' isn't readable (as String) : {:?} (err : {})", self, error);
            }
        }
    }

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as Element)
    fn get_file_content_as_element(&self) -> Element {

        // Get the content
        let content = self.get_file_content();

        // Parsing file content to Element object class
        let element_file : Element = match content.parse() {
            Ok(result_object) => {
                trace!("Parsing Element : {:?}", self);
                result_object
            }
            Err(error) => {
                error!("PANIC_FLM08 - The 'file' isn't parsable : {:?} (err : {})", self, error);
                panic!("PANIC_FLM08 - The 'file' isn't parsable : {:?} (err : {})", self, error);
            }
        };

        // Return result
        element_file
    }

    /// Create the file if don't exist
    fn create_file(&self) -> () {

        // Exit if the file exist
        if self.exists() {
            trace!("File {:?} don't exist (don't create)", self);
            return;
        }

        // Create it
        match File::create(self) {
            Ok(result) => {
                trace!("File {:?} created", self);
                result
            }
            Err(error) => {
                error!("PANIC_FLM09 - The 'file' can't be created : {:?} (err : {})", self, error);
                panic!("PANIC_FLM09 - The 'file' can't be created : {:?} (err : {})", self, error);
            }
        };
    }

    /// Copy a file to a other location
    fn copy_file(&self, to : &Self) -> () {

        let options = fs_extra::file::CopyOptions::new();

        // Copying file
        match fs_extra::file::copy(self, to , &options) {
            Ok(_) => {
                trace!("File {:?} copied", self);
            }
            Err(error) => {
                error!("PANIC_FLM10 - The 'file' can't be copied : {:?} (err : {})", self, error);
                panic!("PANIC_FLM10 - The 'file' can't be copied : {:?} (err : {})", self, error);
            }
        };
    }

    /// Move a file to a other location
    fn move_file(&self, to : &Self) -> () {

        let options = fs_extra::file::CopyOptions::new();

        // Moving file
        match fs_extra::file::move_file(self, to , &options) {
            Ok(_) => {
                trace!("File {:?} moved", self);
            }
            Err(error) => {
                error!("PANIC_FLM11 - The 'file' can't be moved : {:?} (err : {})", self, error);
                panic!("PANIC_FLM11 - The 'file' can't be moved : {:?} (err : {})", self, error);
            }
        };
    }

    /// Delete the file if it exist
    fn delete_file(&self) -> () {

        // Delete file
        match fs_extra::file::remove(self) {
            Ok(_) => {
                trace!("File {:?} deleted", self);
            }
            Err(error) => {
                error!("PANIC_FLM12 - The 'file' can't be deleted : {:?} (err : {})", self, error);
                panic!("PANIC_FLM12 - The 'file' can't be deleted : {:?} (err : {})", self, error);
            }
        };
    }
}

impl FileManager for &str {
    ///  Return the content of the folder (as ReadDir)
    fn get_folder_content(&self) -> ReadDir {
        Path::new(self).get_folder_content()
    }

    /// Create the folder if don't exist
    fn create_folder(&self) -> ()  {
        Path::new(self).create_folder()
    }

    /// Copy each item of a folder to a other
    fn copy_folder(&self, to : &Self) -> () {
        Path::new(self).copy_folder(Path::new(to))
    }

    /// Move each item of a folder to a other
    fn move_folder(&self, to : &Self) -> () {
        Path::new(self).move_folder(Path::new(to))
    }

    /// Delete the folder if it exist
    fn delete_folder(&self, empty_only : bool) -> () {
        Path::new(self).delete_folder(empty_only)
    }

    /// Check if the file exist, and if it's file, and if it's readable and return it (as File)
    fn get_file(&self) -> File {
        Path::new(self).get_file()
    }

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as String)
    fn get_file_content(&self) -> String {
        Path::new(self).get_file_content()
    }

    /// Check if the file exist, and if it's file, and if it's readable and return this content (as Element)
    fn get_file_content_as_element(&self) -> Element {
        Path::new(self).get_file_content_as_element()
    }

    /// Create the file if don't exist
    fn create_file(&self) -> () {
        Path::new(self).create_file()
    }

    /// Copy a file to a other location
    fn copy_file(&self, to : &Self) -> () {
        Path::new(self).copy_file(Path::new(to))
    }

    /// Move a file to a other location
    fn move_file(&self, to : &Self) -> () {
        Path::new(self).move_file(Path::new(to))
    }

    /// Delete the file if it exist
    fn delete_file(&self) -> () {
        Path::new(self).delete_file()
    }
}

impl FileManager for String {
    ///  Return the content of the folder (as ReadDir)
    fn get_folder_content(&self) -> ReadDir {
        Path::new(self).get_folder_content()
    }

    /// Create the folder if don't exist
    fn create_folder(&self) -> () {
        Path::new(self).create_folder()
    }

    /// Copy each item of a folder to a other
    fn copy_folder(&self, to : &Self) -> () {
        Path::new(self).copy_folder(Path::new(to))
    }

    /// Move each item of a folder to a other
    fn move_folder(&self, to : &Self) -> () {
        Path::new(self).move_folder(Path::new(to))
    }

    /// Delete the folder if it exist
    fn delete_folder(&self, empty_only : bool) -> () {
        Path::new(self).delete_folder(empty_only)
    }

    /// Check if the file exist, and if it's file, and if it's readable and return it (as File)
    fn get_file(&self) -> File {
        Path::new(self).get_file()
    }

    /// Check if the file exist, and if it's folder, and if it's readable and return this content (as String)
    fn get_file_content(&self) -> String {
        Path::new(self).get_file_content()
    }

    /// Check if the file exist, and if it's folder, and if it's readable and return this content (as Element)
    fn get_file_content_as_element(&self) -> Element {
        Path::new(self).get_file_content_as_element()
    }

    /// Create the file if don't exist
    fn create_file(&self) -> () {
        Path::new(self).create_file()
    }

    /// Copy a file to a other location
    fn copy_file(&self, to : &Self) -> () {
        Path::new(self).copy_file(Path::new(to))
    }

    /// Move a file to a other location
    fn move_file(&self, to : &Self) -> () {
        Path::new(self).move_file(Path::new(to))
    }

    /// Delete the file if it exist
    fn delete_file(&self) -> () {
        Path::new(self).delete_file()
    }
}

#[cfg(test)]
mod tests {
    use super::FileManager;
    use super::Path;

    #[test]
    fn module_flm_01_get_folder_content() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_01_get_folder_content";
        folder.get_folder_content();
        // As String
        let folder = String::from(folder);
        folder.get_folder_content();
        // As Path
        let folder = Path::new(&folder);
        folder.get_folder_content();
    }

    #[test]
    fn module_flm_02_create_folder_and_delete_folder() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_02_create_folder_and_delete_folder/to_create";
        folder.create_folder();
        folder.get_folder_content();
        folder.delete_folder(true);
        // As String
        let folder = String::from(folder);
        folder.create_folder();
        folder.get_folder_content();
        folder.delete_folder(true);
        // As Path
        let folder = Path::new(&folder);
        folder.create_folder();
        folder.get_folder_content();
        folder.delete_folder(true);
    }

    #[test]
    fn module_flm_03_copy_folder_and_delete_folder() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_03_copy_folder_and_delete_folder/from";
        let to = "tests/module_file_manager/module_flm_03_copy_folder_and_delete_folder/to";
        to.delete_folder(false);
        to.create_folder();
        folder.copy_folder(&to);
        to.get_folder_content();
        // As String
        let folder = String::from(folder);
        let to = String::from(to);
        to.delete_folder(false);
        to.create_folder();
        folder.copy_folder(&to);
        to.get_folder_content();
        // As Path
        let folder = Path::new(&folder);
        let to = Path::new(&to);
        to.delete_folder(false);
        to.create_folder();
        folder.copy_folder(&to);
        to.get_folder_content();
    }

    #[test]
    fn module_flm_04_move_folder_and_move_folder() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_04_move_folder_and_move_folder/from";
        let to = "tests/module_file_manager/module_flm_04_move_folder_and_move_folder/to";
        to.delete_folder(false);
        to.create_folder();
        folder.move_folder(&to);
        to.get_folder_content();
        to.move_folder(&folder);
        folder.get_folder_content();
        // As String
        let folder = String::from(folder);
        let to = String::from(to);
        to.delete_folder(false);
        to.create_folder();
        folder.move_folder(&to);
        to.get_folder_content();
        to.move_folder(&folder);
        folder.get_folder_content();
        // As Path
        let folder = Path::new(&folder);
        let to = Path::new(&to);
        to.delete_folder(false);
        to.create_folder();
        folder.move_folder(&to);
        to.get_folder_content();
        to.move_folder(&folder);
        folder.get_folder_content();
    }

    #[test]
    fn module_flm_05_get_file() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_05_get_file/file_to_read.txt";
        folder.get_file();
        // As String
        let folder = String::from(folder);
        folder.get_file();
        // As Path
        let folder = Path::new(&folder);
        folder.get_file();
    }

    #[test]
    fn module_flm_06_get_file_content() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_06_get_file_content/file_to_read.txt";
        folder.get_file_content();
        // As String
        let folder = String::from(folder);
        folder.get_file_content();
        // As Path
        let folder = Path::new(&folder);
        folder.get_file_content();
    }

    #[test]
    fn module_flm_07_get_file_content_as_element() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_07_get_file_content_as_element/file_to_read.txt";
        folder.get_file_content_as_element();
        // As String
        let folder = String::from(folder);
        folder.get_file_content_as_element();
        // As Path
        let folder = Path::new(&folder);
        folder.get_file_content_as_element();
    }

    #[test]
    fn module_flm_08_create_file_and_delete_file() {
        // As &str
        let folder = "tests/module_file_manager/module_flm_08_create_file_and_delete_file/file_to_create.txt";
        folder.create_file();
        folder.get_file_content();
        folder.delete_file();
        // As String
        let folder = String::from(folder);
        folder.create_file();
        folder.get_file_content();
        folder.delete_file();
        // As Path
        let folder = Path::new(&folder);
        folder.create_file();
        folder.get_file_content();
        folder.delete_file();
    }

}
