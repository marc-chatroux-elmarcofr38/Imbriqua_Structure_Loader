Tools for cargo checking of a library folder

# How to use

Allowing to checking (with cargo) a Rust library file or a Rust executable file, previously genereted.

With __purge_folder__, this module provide purging folder (as example, before copying content).

With __copy_folder__, this module provide copying file and folder with availibility check.

With __check_result__, this module provite __cargo__ command call (test, doc and other).

## Minimal usecase

In the case of yout making a package __Project_A__ gererating a Rust library file to test, the TODO list is :
* Create a minimal cargo library package (like Imbriqua_Structure_Result, see file tree)
* Create code generation script (Project_A objective) writing result in lib.rs file of Project_B
* In main of Project_A, use check_result function to test Project_B with cargo

### __/Project_A/main.rs__

```rust
mod module_output_checker;

fn main() {

    fn generate_code ("../Project_B/src/lib.rs") {
        // script part generating file in relative path "../Project_B/src/lib.rs"
    }
    
    module_output_checker::check_result("../Project_B/");
}
```

### Bash cargo equivalent

```bash
$ cargo test --manifest-path=".../Project_B/" --all-features --no-run --lib
$ cargo doc --manifest-path=".../Project_B/" --no-deps
```

### File tree

```text
.
├── Project_A/ (executable package, like Imbriqua_Structure_Loader)
│   ├── README.md
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── src/
│   │   ├── main.rs
│   │   ├── module_output_checker.rs
│   │   └── ...
│   └── ...
│
├── Project_B/ (library package to check, like Imbriqua_Structure_Result)
│   ├── README.md
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── src/
│   │   └── lib.rs
│   └── ...
│
└── ...

PS : Of course, you can have a similar folder tree for executable package check
```


## Advenced usecase

In the case of yout making a package __Project_A__ gererating a Rust library file to test, the TODO list is :
* Create a minimal cargo library package (like Imbriqua_Structure_Result, see file tree)
* Create code generation script (Project_A objective) writing result in a output folder, classified by run time (like versioning folder or archives)
* In main of Project_A, use purge_folder function to remove all files in source folder of Project_B (removing old file name)
* In main of Project_A, use copy_folder function to copying last result files in source folder of Project_B
* In main of Project_A, use check_result function to test Project_B with cargo

### __/Project_A/main.rs__

```rust
mod module_output_checker;

fn main() {

    fn generate_code () {
        // script part generating file and complex folder
        // in relative path "../Output_file/{time_name}/src"
    }
    
    let output_path = format!("../Output_folder/{}", time_name);

    // cargo clean, 
    module_output_checker::clean_target_result("../Project_B/");

    // copying and checking
    module_output_checker::purge_folder("../Project_B/src/");
    module_output_checker::copy_folder(output_path.as_str(), "../Project_B/src/");
    module_output_checker::check_result("../Project_B/");

    // cargo build, for example
    module_output_checker::cargo_custom_command(vec!["build"], "../Project_B/");
}
```

### Bash cargo equivalent

```bash
$ cargo test --manifest-path=".../Project_B/" --all-features --no-run --lib
$ cargo doc --manifest-path=".../Project_B/" --no-deps
```

### File tree

```text
.
├── Imbriqua_Structure_Loader/ (executable package)
│   ├── README.md
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── src/
│   │   ├── main.rs
│   │   ├── module_output_checker.rs
│   │   └── ...
│   └── ...
│
├── Imbriqua_Structure_Result/ (library package or executable package)
│   ├── README.md
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── src/
│   │   ├── lib.rs
│   │   └── lib_folder/ 
│   │       └── ...
│   └── ...
│
├── Output_folder/
│   ├── 2024-04-13_12h36m50/
│   │   ├── lib.rs
│   │   └── lib_folder/
│   │       └── ...
│   ├── 2024-04-12_08h47m01/
│   │   ├── lib.rs
│   │   └── lib_folder/
│   │       └── ...
│   └── ...
│
└── ...

PS : Of course, you can have a similar folder tree for executable package check
```

# Panic and failure

* PANIC_OUT01 - The folder don't exist during purge
    * Context : __module_output_checker.rs/purge_folder()__
    * Info : Can't find provided folder
    * Cause : see details in logs file to get :
        * Value of folder path

* PANIC_OUT02 - The folder isn't readable during purge
    * Context : __module_output_checker.rs/purge_folder()__
    * Info : Can't read provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::path::Path::read_dir()__

* PANIC_OUT03 - The 'from' folder don't exist (copying)
    * Context : __module_output_checker.rs/copy_folder()__
    * Info : Can't find provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::path::Path::exist()__

* PANIC_OUT04 - The 'to' folder don't exist (copying)
    * Context : __module_output_checker.rs/copy_folder()__
    * Info : Can't find provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::path::Path::exist()__

* PANIC_OUT05 - The folder isn't readable (copying)
    * Context : __module_output_checker.rs/copy_folder()__
    * Info : Can't read provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::path::Path::read_dir()__

* PANIC_OUT06 - Can't copying folder
    * Context : __module_output_checker.rs/copy_folder()__
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::fs_extra::copy()__

* PANIC_OUT07 - Can't copying file
    * Context : __module_output_checker.rs/copy_folder()__
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::fs::copy()__

* PANIC_OUT08 - Error in ReadDir iterator
    * Context : __module_output_checker.rs/copy_folder()__
    * Info : Can't copy provided entry
    * Cause : see details in logs file to get :
        * error informations of __ReadDir::Iterator__

* PANIC_OUT09 - Error in OsString::to_str()
    * Context : __module_output_checker.rs/copy_folder()__
    * Info : Can't copy provided entry
    * Cause : see details in logs file to get :
        * Value of entry (debuging syntax)
        * documentation of ReadDir::Iterator

* PANIC_OUT10 - Error in DirEntry::file_type()
    * Context : __module_output_checker.rs/copy_folder()__
    * Info : Can't copy provided entry
    * Cause : see details in logs file to get :
        * error informations of __DirEntry::file_type__

* WARN_OUT01 - Error in ReadDir iterator
    * Context : __module_output_checker.rs/purge_folder()__
    * Info : Can't remove provided entry
    * Cause : see details in documentation of ReadDir::Iterator

* WARN_OUT02 - Error in pathBuf::to_str()
    * Context : __module_output_checker.rs/purge_folder()__
    * Info : Can't remove provided entry
    * Cause : see details in documentation of PathBuf::to_str

* WARN_OUT03 - Error in removing entry
    * Context : __module_output_checker.rs/purge_folder()__
    * Info : Can't remove provided entry
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of __std::fs_extra::remove_items()__

* WARN_OUT04 - Cound't get output
    * Context : __module_output_checker.rs/represent_command_output()__
    * Info : Can't get output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of __std::process::Command::output()__

* WARN_OUT05 - Couldn't get STDOUT
    * Context : __module_output_checker.rs/represent_command_output()__
    * Info : Can't print output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of __std::str::from_utf8()__

* WARN_OUT06 - Couldn't get STDERR
    * Context : __module_output_checker.rs/represent_command_output()__
    * Info : Can't print output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of __std::str::from_utf8()__ 
