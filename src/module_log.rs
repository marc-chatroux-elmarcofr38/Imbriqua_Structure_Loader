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

//! # eee
//! 
//! eeeee
//! eeeee
//! 
//! ```
//! # fn f() {
//! use module_log.rs;
//! 
//! fn main() {
//! 
//!     let handle = open_module();
//! 
//! }
//! # }
//! # fn main() {}
//! ```
//! 

use anyhow::Result;
use log::{info, warn, LevelFilter};
use log4rs::init_config;
use log4rs::Handle;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, RawConfig, Config, Deserializers, Logger, Root};
use serde_yaml::from_str;

pub fn open_module() -> Handle {
    /*
        main function for configuration of gloabal logger
        Try to load the configuration file, else try to load a backup configuration

        Try to load a backup configuration for global logger.
        Use a "hard writted" configuration

        Try to load a default configuration for global logger.
        It use the following file : "config_log.yml"
    */


    // Itinialisation of global logger with backup config
    let config : Config = match get_config_by_backup() {
        Ok(result) => {result},
        Err(_) => {panic!("PANIC_LOG01 - Error during the loading on logs modules")},
    };

    let handle : Handle = match init_config(config) {
        Ok(result) => {result},
        Err(_) => {panic!("PANIC_LOG02 - Error during the loading on logs modules")},
    };

    // Get file config
    let config : Config = match get_config_by_file() {
        Ok(result) => {
            result
        },
        Err(error) => {
            warn!("WARN_LOG01 - Error during default configuration loading \"{}\": {}", "config_log.yml", error);
            info!("Logger init success : use of \"{}\"", "!!! BACKUP CONFIGURATION !!!");
            return handle
        },
    };

    // Itinialisation of global logger with file config
    handle.set_config(config);

    // Get config
    handle
}

fn get_config_by_file() -> Result<Config> {
    /*
        Define a config by loading "config_log.yml"
    */

    // Loading the file
    let default_config_str : &'static str = include_str!("config_log.yml");

    // Deserialize
    let config : RawConfig = from_str(default_config_str)?;
    let (appenders, errors) = config.appenders_lossy(&Deserializers::default());
    
    // Error test
    if !errors.is_empty() {
        return Err(anyhow::Error::new(errors));
    }

    // Initialise config object
    let config : Config = Config::builder()
        .appenders(appenders)
        .loggers(config.loggers())
        .build(config.root())?;

    Ok(config)
}

fn get_config_by_backup() -> Result<Config> {
    //!
    //! dfbdfxb
    //! 
    //! 
    //! dehdxg
    //! 
    /*
        Define a backup config
    */

    // Setup of console logging output
    let stdout : ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{M}: {m} {n}")))
        .build();

    // Setup of file logging tools
    let requests : FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}")))
        .build("imbriqua_structure.log")?;

    // Setup of global logger
    let config : Config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().appender("stdout").build("app::stdout", LevelFilter::Debug))
        .logger(Logger::builder().appender("requests").build("app::requests", LevelFilter::Debug))
        .build(Root::builder().appender("stdout").appender("requests").build(LevelFilter::Debug))?;

    Ok(config)
}

#[test]
fn check_configuration_by_backup() {

    // Checking execution
    let result = get_config_by_backup();
    
    // Checking Result
    assert!(result.is_ok());
}

#[test]
fn check_configuration_by_file() {

    // Checking execution
    let result = get_config_by_file();
    
    // Checking Result
    assert!(result.is_ok());
}