use anyhow::Result;
use log::{info, warn, LevelFilter};
use log4rs::init_config;
use log4rs::Handle;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, RawConfig, Config, Deserializers, Logger, Root};
use serde_yaml::from_str;

pub fn open_module() -> (Handle, Config, bool) {
    /*
        main function for configuration of gloabal logger
        Try to load the configuration file, else try to load a backup configuration
    */

    match load_configuration_by_file() {
        Ok(result) => {
            info!("Logger init success : use of \"{}\"", "config_log.yml");
            result
        },
        Err(error) => {
            match load_configuration_by_backup() {
                Ok(result) => {
                    warn!("WARN_LOG01 - Error during default configuration loading \"{}\": {}", "config_log.yml", error);
                    info!("Logger init success : use of \"{}\"", "!!! BACKUP CONFIGURATION !!!");
                    result
                },
                Err(_) => {
                    panic!("PANIC_LOG01 - Error during the loading on logs modules")
                },
            }
        },
    }
}

fn load_configuration_by_file() -> Result<(log4rs::Handle, Config, bool)> {
    /*
        Try to load a default configuration for global logger.
        It use the following file : "config_log.yml"
    */

    // Get config
    let config : Config = get_config_by_file()?;

    // Itinialisation of global logger
    let handle : Handle = init_config(config)?;

    // Get config
    let config : Config = get_config_by_file()?;

    Ok((handle, config, false))
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

fn load_configuration_by_backup() -> Result<(log4rs::Handle, Config, bool)> {
    /*
        Try to load a backup configuration for global logger.
        Use a "hard writted" configuration
    */

    // Get backup config
    let config : Config = get_config_by_backup()?;

    // Itinialisation of global logger
    let handle : Handle = init_config(config)?;

    // Get backup config
    let config : Config = get_config_by_backup()?;

    Ok((handle, config, true))
}

fn get_config_by_backup() -> Result<Config> {
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
fn check_load_configuration_by_file() {

    // Checking execution
    let result = load_configuration_by_file();

    // Checking Result
    assert!(result.is_ok());

    // Checking len and type
    let (_unpack_1, _unpack_2, _unpack_3) : (log4rs::Handle, Config, bool) = result.unwrap();
}

#[test]
fn check_load_configuration_by_backup() {

    // Checking execution
    let result = load_configuration_by_backup();
    
    // Checking Result
    assert!(result.is_ok());

    // Checking len and type
    let (_unpack_1, _unpack_2, _unpack_3) : (log4rs::Handle, Config, bool) = result.unwrap();
}