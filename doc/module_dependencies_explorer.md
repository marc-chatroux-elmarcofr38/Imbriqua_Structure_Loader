Short description

# How to use

...

## Minimal usecase

...

## Advanced usecase

...

# Panic and failure

* PANIC_DEP01 - Unloaded dependencies : suspicious of circular dependencies
    * Context : [`LoadingTracker::prepare`]
    * Info : One of PackageImport return a package with parent link, circular loading issue (A import B, and B import A)
    * Cause : see details in logs file to get :
        * Files names (child and parent)

* PANIC_DEP02 - packageImport element without importedPackage child
    * Context : [`LoadingTracker::add_dependencies`]
    * Info : a packageImport Element haven't the needed pattern, not importedPackage child
    * Cause : see details in logs file to get :
        * File and package targeted

* PANIC_DEP03 - importedPackage element without href attribute
    * Context : [`LoadingTracker::add_dependencies`]
    * Info : a importedPackage Element haven't the needed pattern, not href attribute
    * Cause : see details in logs file to get :
        * File and package targeted

* PANIC_DEP04 - href attribute without '#' separator : package
    * Context : [`LoadingTracker::add_dependencies`]
    * Info : a href attribute haven't the needed pattern, no '#' separator
    * Cause : see details in logs file to get :
        * File and package targeted
