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

//! Configure resilient logger : this module provide logger config using two configuration
//! 
//! # How to use
//! 
//! Configure logger with YAML configuration file, after configuring logger with backup configuration (described in next section)
//! 
//! This two-step provide a working detailed logging backup configuration during configuration file error
//! 
//! ## Minimal usecase
//! 
//! ```
//! # fn f() {
//! mod module_log;
//! use log::{error, warn, info, debug, trace};
//! 
//! fn main() {
//! 
//!     let _handle = module_log::open_module();
//! 
//!     error!("It's an error log");
//!     warn!("It's a warn log");
//!     info!("It's an info log");
//!     debug!("It's a debug log");
//!     trace!("It's a trace log");
//! 
//! }
//! # }
//! # fn main() {}
//! ```
//! 
//! ## Customization
//! 
//! Edit config_log.yaml file configuration this log4rs notation
//! 
//! ## Example config_log.yaml file configuration
//! 
//! ```
//! appenders:
//!     stdout:
//!         kind: console
//!         encoder:
//!             pattern: "{M}: {m} {n}"
//!         filters:
//!             - kind: threshold
//!               level: info
//! 
//!     requests:
//!         kind: file
//!         path: "imbriqua_structure_loader.log"
//!         encoder:
//!             pattern: "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}: {m} {n}"
//!         filters:
//!             - kind: threshold
//!               level: trace
//! 
//! root:
//!     level: trace
//!     appenders:
//!         - stdout
//!         - requests
//! ```
//! 
//! ## Equivalent YAML file of backup configuration
//! 
//! ```
//! appenders:
//!     stdout:
//!         kind: console
//!         encoder:
//!             pattern: "! BACKUP_LOGGER ! {M}: {m} {n}"
//!         filters:
//!             - kind: threshold
//!               level: trace
//! 
//!     requests:
//!         kind: file
//!         path: "imbriqua_structure_loader.log"
//!         encoder:
//!             pattern: "{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}"
//!         filters:
//!             - kind: threshold
//!               level: trace
//! 
//! root:
//!     level: trace
//!     appenders:
//!         - stdout
//!         - requests
//! ```
//! 
//! # Panic and failure
//! 
//! * WARN_LOG01 - Error during default configuration loading
//!     * Context : __module_log.rs/open_modules()__
//!     * Info : The file __config_log.yml__ can't be loaded in log4rs configuration
//!     * Info : A backup logging configuration may be load,logs are in __imbriqua_structure_loader.log__ file
//!     * Cause : See logs for syntaxe error details, or deserialize error details
//! 
//! * PANIC_LOG01 - Error during the loading on logs modules
//!     * Context : __module_log.rs/open_modules()__
//!     * Info : No logs are provided, so, make panic
//!     * Cause : The error come from __get_config_by_backup()__ function
//! 
//! * PANIC_LOG02 - Error during the loading on logs modules
//!     * Context : __module_log.rs/open_modules()__
//!     * Info : No logs are provided, so, make panic
//!     * Cause : The error come from __log4rs::init_config()__ function
//! 

use anyhow::Result;
use log::{info, warn, error, LevelFilter};
use log4rs::init_config;
use log4rs::Handle;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, RawConfig, Config, Deserializers, Logger, Root};
use serde_yaml::from_str;

pub fn open_module() -> Handle {
    //! Configure logger with config_log.yml, after configure it with a backup configuation
    //! 
    //! Try to load a backup configuration ("hard writted" configuration)
    //! After, try to load "config_log.yml" configuration

    // Itinialisation of global logger with backup configuration
    let config : Config = match get_config_by_backup() {
        Ok(result) => {result},
        Err(error) => {
            error!("PANIC_LOG01 - Error during the loading on logs modules - {}", error);
            panic!("PANIC_LOG01 - Error during the loading on logs modules")},
        
    };
    let handle : Handle = match init_config(config) {
        Ok(result) => {result},
        Err(error) => {
            error!("PANIC_LOG01 - Error during the loading on logs modules - {}", error);
            panic!("PANIC_LOG02 - Error during the loading on logs modules")
        },
    };

    // Itinialisation of global logger with "config_log.yml" configuration
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
    handle.set_config(config);

    info!("Log handle loaded");

    // Get config
    handle
}

fn get_config_by_file() -> Result<Config> {
    //! Define a config by loading "config_log.yml"

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
    //! Define a backup config (hard-writted)

    // Setup of console logging output
    let stdout : ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("! BACKUP_LOGGER ! {M}: {m} {n}")))
        .build();

    // Setup of file logging tools
    let requests : FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}")))
        .build("imbriqua_structure_loader.log")?;

    // Setup of global logger
    let config : Config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().appender("stdout").build("app::stdout", LevelFilter::Trace))
        .logger(Logger::builder().appender("requests").build("app::requests", LevelFilter::Trace))
        .build(Root::builder().appender("stdout").appender("requests").build(LevelFilter::Trace))?;

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