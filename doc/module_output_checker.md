Tools for cargo checking of a library folder

# How to use

Allowing to checking (with cargo) a Rust library file or a Rust executable file, previously genereted.

With [`open_link`] and method on [`PackageLink`], this module provide cargo command execution on defined package folder, like :

* [`PackageLink::cargo_integrity_check`] : link validation with `cargo locate-project` (check if Cargo.toml file exist)
* [`PackageLink::cargo_clean`] : delete __target/__ folder with `cargo clean`
* [`PackageLink::cargo_full_check`] : package validation with :
    * `cargo check --all-features`
    * `cargo test --all-features --no-run`
    * `cargo build --all-features`
    * `cargo doc --no-deps`
* [`PackageLink::cargo_custom_command`] : run custom `cargo` command, using a list of argument (function and options)

Also, [`PackageLink`] provide build-in folder management shortcut :

* [`PackageLink::get_absolute_cargo_path`] : return the path of __Cargo.toml__
* [`PackageLink::get_absolute_source_path`] : return the path of __src/__
* [`PackageLink::purge_source`] : remove all content of the folder __src/__
* [`PackageLink::load_from`] : copy all content of a output folder in  __src/__

## Minimal usecase

In the case of yout making a package __Project_A__ gererating a Rust library file to test, the TODO list is :
* Create a minimal cargo library package (like Imbriqua_Structure_Result, see file tree)
* Create code generation script (Project_A objective) writing result in lib.rs file of Project_B
* In main of Project_A, use use [`PackageLink::cargo_full_check`] function to test Project_B with cargo

### __/Project_A/main.rs__

```rust
mod module_output_checker;

fn main() {

    fn generate_code ("../Project_B/src/lib.rs") {
        // script part generating file in relative path "../Project_B/src/lib.rs"
    }

    // Instantiation of PackageLink
    let link = module_output_checker::open_link("../Project_B/Cargo.toml");
    // Make full check
    assert_eq!(link.cargo_full_check(), true);
}
```

### Bash cargo equivalent

```bash
$ cargo check --manifest-path=".../Project_B/Cargo.toml" --all-features
$ cargo test --manifest-path=".../Project_B/Cargo.toml" --all-features --no-run
$ cargo build --manifest-path=".../Project_B/Cargo.toml" --all-features
$ cargo doc --manifest-path=".../Project_B/Cargo.toml" --no-deps
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
* Create a minimal cargo library package folder (like Imbriqua_Structure_Result, see file tree)
* Create code generation script (Project_A objective) writing result in a output folder, classified by run time (like module_file_env)
* In main of Project_A, use [`PackageLink`] like the following example (or with custom command)

### __/Project_A/main.rs__

```rust
mod module_output_checker;

fn main() {

    // Set custom output folder
    let time_name: String = chrono::Local::now().format("%Y-%m-%d_%Hh%Mm%S/").to_string();
    let output_path = format!("../Output_folder/{}", time_name);

    fn generate_code (output_path) {
        // script part generating file and complex folder
        // in relative path "../Output_file/{time_name}/src"
    }

    // Instantiation of PackageLink
    let link = module_output_checker::open_link("../Project_B/Cargo.toml");

    // cargo clean
    assert_eq!(link.cargo_clean(), true);

    // copying and checking
    link.purge_source();
    link.load_from(Path::new(output_path));
    assert_eq!(link.cargo_full_check(), true);

    // Custom command, for example 'cargo run'
    assert_eq!(link.cargo_custom_command(vec!["run"]), true);
}
```

### Bash cargo equivalent

```bash
$ cargo clean --manifest-path=".../Project_B/Cargo.toml"
$ cargo check --manifest-path=".../Project_B/Cargo.toml" --all-features
$ cargo test --manifest-path=".../Project_B/Cargo.toml" --all-features --no-run
$ cargo build --manifest-path=".../Project_B/Cargo.toml" --all-features
$ cargo doc --manifest-path=".../Project_B/Cargo.toml" --no-deps
$ cargo run --manifest-path=".../Project_B/Cargo.toml"
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

* PANIC_OUT01 - Can't find cargo.toml
    * Context : [`PackageLink::cargo_integrity_check`]
    * Info : Can't pass check of provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of [__cargo locate-project__](https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html)

* PANIC_OUT02 - Can't clean
    * Context : [`PackageLink::cargo_clean`]
    * Info : Can't clean provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of [__cargo clean__](https://doc.rust-lang.org/cargo/commands/cargo-clean.html)

* WARN_OUT01 - Cound't get output
    * Context : [`represent_command_output`]
    * Info : Can't get output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of [__std::process::Command::output()__](https://doc.rust-lang.org/std/process/struct.Command.html#method.output)

* WARN_OUT02 - Couldn't get STDOUT
    * Context : [`represent_command_output`]
    * Info : Can't print output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of [__std::str::from_utf8()__](https://doc.rust-lang.org/std/str/fn.from_utf8.html)

* WARN_OUT03 - Couldn't get STDERR
    * Context : [`represent_command_output`]
    * Info : Can't print output information
    * Cause : see details in logs file to get :
        * the command
        * error informations of [__std::str::from_utf8()__](https://doc.rust-lang.org/std/str/fn.from_utf8.html)
