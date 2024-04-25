Short description

# How to use

...

## Minimal usecase

...

## Advanced usecase

...

# Panic and failure

* PANIC_DEP01 - Unloaded dependencies : suspicious of circular dependencies
    * Context : [`PackageLoader::prepare`]
    * Info : One of PackageImport return a package with parent link, circular loading issue (A import B, and B import A)
    * Cause : see details in logs file to get :
        * Files names (child and parent)

* PANIC_DEP02 - packageImport element without importedPackage child
    * Context : [`PackageLoader::add_dependencies`]
    * Info : Can't pass check of provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of [__cargo locate-project__](https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html)

* PANIC_DEP03 - importedPackage element without href attribute
    * Context : [`PackageLoader::add_dependencies`]
    * Info : Can't pass check of provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of [__cargo locate-project__](https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html)

* PANIC_DEP04 - href attribute without '#' separator : package
    * Context : [`PackageLoader::add_dependencies`]
    * Info : Can't pass check of provided folder
    * Cause : see details in logs file to get :
        * Value of folder path
        * error informations of [__cargo locate-project__](https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html)



* PANIC_###0# - ...
    * Context : __###.rs/###()__
    * Info : ...
    * Cause : ...
