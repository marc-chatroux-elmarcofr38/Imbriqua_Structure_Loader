Tools for other "writing" tools : including homogenous naming function and pre_calculation object

# Naming tools (homogenous)

Following Traits provide homogenous for naming path, naming struct and naming link
 - [`NamingLink`], for providing link name [`EnumOwnedMember`] in package [`LoadingPackage`], as hierarchical position
 - [`NamingPath`], for providing path to [`LoadingPackage`], [`ImportedPackage`] and [`EnumOwnedMember`]
 - [`NamingStruct`], for providing struct name to [`EnumOwnedMember`]

## WritingPath

...

## LoadingTracker

...

## Writting Organiser

Following Traits provide organisation for writting output Rust file

For writting __lib.rs__: used in [`writing_lib_file`][crate::writing_lib_file]
 - [`WritingLibFileHead`] : 


For writting __${package}/mod.rs__: used in [`writing_mod_file`][crate::writing_mod_file]
 - [`WritingModFileHead`] : 
 - [`WritingModFileObjectSection`] : 

For writting __${package}/${owned_member}.rs__: used in [`writing_mod_object`][crate::writing_mod_object]
 - [`WritingCallModObject`] : 
 - [`WritingModObject`] : 
 - [`WritingModValidation`] : 



## Advanced usecase

...

# Panic and failure

* PANIC_###0# - ...
    * Context : __###.rs/###()__
    * Info : ...
    * Cause : ...
