Provide shortcut to filesystem function, with error forwarding and panic control

# How to use

Adding __FileManager__ traits to __std::fs::Path__

This traits add to __std::fs::Path__ the following function to folder :

* [`FileManager::check_is_dir`] : Panic if it's not a directory (used by other function)
* [`FileManager::get_folder_content`] : Return the content of the folder (as ReadDir)
* [`FileManager::create_folder`] : Create the folder if don't exist
* [`FileManager::copy_folder`] : Copy each item of a folder to a other
* [`FileManager::move_folder`] : Move each item of a folder to a other
* [`FileManager::delete_folder`] : Delete the folder if it exist
* [`FileManager::purge_folder`] : Remove all content of the folder if it exist

This traits add to __std::fs::Path__ the following function to file :

* [`FileManager::check_is_file`] : Panic if it's not a file (used by other function)
* [`FileManager::write_new_file`] : Check if the file exist, and if it's file, and if it's readable and return it (as File)
* [`FileManager::get_file_content`] : Check if the file exist, and if it's file, and if it's readable and return this content (as String)
* [`FileManager::copy_file`] : Copy a file to a other location
* [`FileManager::move_file`] : Move a file to a other location
* [`FileManager::delete_file`] : Delete the file if it exist

This traits add to __std::fs::Path__ the following function to all path :

* [`FileManager::canonicalize_pathbuf`] : Canonicalize the path if it exist

## Good practice

Use [`crate::custom_file_tools`] in each rust file, replacing `Path`, and `PathBuf`, with the following command :

* __main.rs__ : `pub mod custom_file_tools;`
* random file : `use crate::custom_file_tools::*;`

This practice allowing you to use new traits of [`Path`] and [`PathBuf`]

```rust
// main.rs
pub mod custom_file_tools;

fn main () {
    // ...
}
```

```rust
// foo.rs
use crate::custom_file_tools__::{FileManager, Path, PathBuf};

fn bar () {
    // ...

    let foo = Path::new("./foo");
    foo.check_is_dir();

    // ...
}

```

# Panic and failure

* PANIC_FLM01 - The 'folder' isn't a directory (or don't exist)
    * Context : [`FileManager::check_is_dir`]
    * Info : The path isn't an existing directory
    * Cause :
        * see details in logs file to get the __Path__
        * The error come from [__Path::is_dir()__](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_dir) function

* PANIC_FLM02 - The 'folder' isn't readable
    * Context : [`FileManager::get_folder_content`]
    * Info : The folder isn't readable
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::dir::get_dir_content2()__](https://docs.rs/fs_extra/latest/fs_extra/dir/fn.get_dir_content2.html) function

* PANIC_FLM03 - The 'folder' can't be created
    * Context : [`FileManager::create_folder`]
    * Info : The folder isn't creatable
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::dir::create_all()__](https://docs.rs/fs_extra/latest/fs_extra/dir/fn.create_all.html) function

* PANIC_FLM04 - The 'folder' can't be copied
    * Context : [`FileManager::copy_folder`]
    * Info : The folder isn't copied
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::copy_items()__](https://docs.rs/fs_extra/latest/fs_extra/fn.copy_items.html) function

* PANIC_FLM05 - The 'folder' can't be moved
    * Context : [`FileManager::move_folder`]
    * Info : The folder isn't moved
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::move_items()__](https://docs.rs/fs_extra/latest/fs_extra/fn.move_items.html) function

* PANIC_FLM06 - The 'folder' can't be deleted
    * Context : [`FileManager::delete_folder`]
    * Info : The folder isn't deleted
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::dir::remove()__](https://docs.rs/fs_extra/latest/fs_extra/dir/fn.remove.html) function

* PANIC_FLM07 - The 'folder' can't be purged
    * Context : [`FileManager::purge_folder`]
    * Info : The folder isn't purged
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::dir::remove_items()__](https://docs.rs/fs_extra/latest/fs_extra/fn.remove_items.html) function

* PANIC_FLM08 - The 'file' isn't a file (or don't exist)
    * Context : [`FileManager::check_is_file`]
    * Info : The path isn't an existing file
    * Cause :
        * see details in logs file to get the __Path__
        * The error come from [__Path::is_file()__](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_file) function

* PANIC_FLM09 - The 'file' can't be created
    * Context : [`FileManager::write_new_file`]
    * Info : The file already exist, or can't be created
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__std::fs::File::create()__](https://doc.rust-lang.org/std/fs/struct.File.html#method.create) function, or [__Path.exist(s)__](https://doc.rust-lang.org/std/path/struct.Path.html#method.exists)

* PANIC_FLM10 - The 'file' isn't readable (as String)
    * Context : [`FileManager::get_file_content`]
    * Info : The file isn't readed
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::file::read_to_string()__](https://docs.rs/fs_extra/latest/fs_extra/file/fn.read_to_string.html) function

* PANIC_FLM12 - The 'file' can't be copied
    * Context : [`FileManager::copy_file`]
    * Info : The file isn't copied
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::file::copy()__](https://docs.rs/fs_extra/latest/fs_extra/file/fn.copy.html) function

* PANIC_FLM13 - The 'file' can't be moved
    * Context : [`FileManager::move_file`]
    * Info : The file isn't moved
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::file::move_file()__](https://docs.rs/fs_extra/latest/fs_extra/file/fn.move_file.html) function

* PANIC_FLM14 - The 'file' can't be deleted
    * Context : [`FileManager::delete_file`]
    * Info : The file isn't deleted
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__fs_extra::file::remove()__](https://docs.rs/fs_extra/latest/fs_extra/file/fn.remove.html) function

* PANIC_FLM15 - Can't canonicalize
    * Context : [`FileManager::canonicalize_pathbuf`]
    * Info : The path isn't canonicalizable
    * Cause :
        * see details in logs file to get the __Path__ and the error
        * The error come from [__std::fs::canonicalize()__](https://doc.rust-lang.org/std/fs/fn.canonicalize.html) function
