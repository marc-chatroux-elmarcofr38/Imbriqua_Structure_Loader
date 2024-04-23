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


## Advanced usecase

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

* PANIC_OUT02 - PANIC_OUT0 - Can't find cargo.toml
    * Context : __module_output_checker.rs/Package_Link::cargo_integrity_check()__
    * Info : Can't pass check of provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of [__cargo locate-project__](https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html)

* PANIC_OUT02 - PANIC_OUT02 - Can't clean
    * Context : __module_output_checker.rs/Package_Link::cargo_clean()__
    * Info : Can't clean provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of [__cargo clean__](https://doc.rust-lang.org/cargo/commands/cargo-clean.html)

* WARN_OUT01 - Cound't get output
    * Context : __module_output_checker.rs/represent_command_output()__
    * Info : Can't get output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of [__std::process::Command::output()__](https://doc.rust-lang.org/std/process/struct.Command.html#method.output)

* WARN_OUT02 - Couldn't get STDOUT
    * Context : __module_output_checker.rs/represent_command_output()__
    * Info : Can't print output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of [__std::str::from_utf8()__](https://doc.rust-lang.org/std/str/fn.from_utf8.html)

* WARN_OUT03 - Couldn't get STDERR
    * Context : __module_output_checker.rs/represent_command_output()__
    * Info : Can't print output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of [__std::str::from_utf8()__](https://doc.rust-lang.org/std/str/fn.from_utf8.html)
