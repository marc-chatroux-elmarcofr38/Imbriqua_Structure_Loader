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
#![warn(rustdoc::missing_doc_code_examples)]

//! This crate is a Rust port of Google's high-performance [SwissTable] hash
//! map, adapted to make it a drop-in replacement for Rust's standard `HashMap`
//! and `HashSet` types.
//!
//! The original C++ version of [SwissTable] can be found [here], and this
//! [CppCon talk] gives an overview of how the algorithm works.
//!
//! [SwissTable]: https://abseil.io/blog/20180927-swisstables
//! [here]: https://github.com/abseil/abseil-cpp/blob/master/absl/container/internal/raw_hash_set.h
//! [CppCon talk]: https://www.youtube.com/watch?v=ncHmEUmJZf4

mod module_log;
mod module_dependencies_explorer;
use log::info;

use std::fs;

// Checking if compiling
mod cmof_module;

fn main() {

    // Initialise global logger
    let _handle = module_log::open_module();

    // Set used folders (input folder and output folder)
    let mut loading_env = module_dependencies_explorer::LoadingTracker::new();

    // Load ordered packages list
    loading_env.import_dependencies_file("BPMNDI.cmof", "_0", "root");

    info!("{}", loading_env);

    loading_env.prebuild("cmof_module.rs");

    let _ = fs::copy(loading_env.file_env.output_subfolder.clone() + "cmof_module.rs", "src/cmof_module.rs");

    // Delete output folder if is empty
    loading_env.close();
}

#[test]
fn le_test() {

    // Initialise global logger
    let _handle = module_log::open_module();

    // Set used folders (input folder and output folder)
    let loading_env = module_dependencies_explorer::LoadingTracker::new();

    // Delete output folder if is empty
    loading_env.close();
}