Tools for other "writing" tools : including homogenous naming function and pre_calculation object

# Naming tools (homogenous)

Following Traits provide homogenous for naming path, naming struct and naming link
 - [`NamingLink`], for providing link name [`EnumOwnedMember`] in package [`LoadingPackage`], as hierarchical position
 - [`NamingStruct`], for providing struct name to [`EnumOwnedMember`]

# Providing full homogenous path to [`LoadingTracker`]

Implement :
 - [`LoadingTracker::get_project_lib_file`] : Get lib.rs file for the LoadingTracker
 - [`LoadingTracker::get_object_file`] : Get ${package}.rs file for a object of a package

# Adding [`LoadingTracker::writing_preparation`]

Build all pre calculing information needed for writting
Stored in [`LoadingTracker::pre_calculation`] ([`LoadingPreCalculation`])

# Writting Organiser

Following Traits provide organisation for writting output Rust file

### For writting __lib.rs__ from [`LoadingPackage`]
 - [`WritingLibFile`] : Trait for writting __lib.rs__ file from entities

&rarr; Used in [`writing_lib_file`][crate::writing_lib_file]

### For writting __${package}/${owned_member}.rs__ from [`LoadingPackage`]
 - [`WritingModObjectCaller`] : Trait for dispatch run for writting __${owned_member}.rs__ file from [`EnumOwnedMember`]
 - [`WritingModObject`] : Trait for writting __${owned_member}.rs__ file from [`EnumOwnedMember`] element
 - [`WritingModValidation`] : Trait for writting __${owned_member}.rs__ struct validation from [`EnumOwnedMember`] element
 - Add part for add calculed import in head of file

&rarr; Used in [`writing_entity`][crate::writing_entity]
