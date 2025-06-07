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
