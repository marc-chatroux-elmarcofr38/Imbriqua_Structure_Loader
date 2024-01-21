use anyhow::Result;
use log::{info, warn, LevelFilter};
//use log4rs::filter::Response
use log4rs::init_config;
use log4rs::Handle;
// use log4rs::init_raw_config;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, RawConfig, Config, Deserializers, Logger, Root};
use serde_yaml::from_str;

pub fn open_module() -> (Handle, Config, bool) {

    // Exit if error
    let result = sub_open_module();
    if result.is_err() {
        panic!("Error during the loading on logs modules")
    };

    result.unwrap()
}

fn sub_open_module() -> Result<(log4rs::Handle, Config, bool)> {
    /*
        main fucntion for configuration of gloabal logger
        Try to load the configuration file, else try to load a backup configuration
    */

    match load_configuration_file() {
        Ok(result) => {
            info!("Logger init success : use of \"{}\"", "config_log.yml");
            Ok(result)
        },
        Err(error) => {
            match load_configuration_backup() {
                Ok(result) => {
                    info!("Logger init success : use of \"{}\"", "!!! BACKUP CONFIGURATION !!!");
                    warn!("Error during default configuration loading : {}", error);
                    Ok(result)
                },
                Err(error) => {
                    panic!("Error during backup configuration loading {}", error);
                },
            }
        },
    }
}

fn load_configuration_file() -> Result<(log4rs::Handle, Config, bool)> {
    /*
        Try to load a default configuration for global logger.
        It use the following file : "config_log.yml"
    */

    // Get config
    let config = get_config_by_file()?;

    // Itinialisation of global logger
    let handle : Handle = init_config(config)?;

    // Get config
    let config = get_config_by_file()?;

    Ok((handle, config, false))
}

fn get_config_by_file() -> Result<Config> {
    /*
        Define a config by loading "config_log.yml"
    */

    // Loading the file
    let default_config_str = include_str!("config_log.yml");

    // Deserialize
    let config : RawConfig = from_str(default_config_str)?;
    let (appenders, errors) = config.appenders_lossy(&Deserializers::default());
    
    // Error test
    if !errors.is_empty() {
        return Err(anyhow::Error::new(errors));
    }

    // Initialise config object
    let config = Config::builder()
        .appenders(appenders)
        .loggers(config.loggers())
        .build(config.root())?;

    Ok(config)
}

fn load_configuration_backup() -> Result<(log4rs::Handle, Config, bool)> {
    /*
        Try to load a backup configuration for global logger.
        Use a "hard writted" configuration
    */

    // Get backup config
    let config = get_backup_config()?;

    // Itinialisation of global logger
    let handle = init_config(config)?;

    // Get backup config
    let config = get_backup_config()?;

    Ok((handle, config, true))
}

fn get_backup_config() -> Result<Config> {
    /*
        Define a backup config
    */

    // Setup of console logging output
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{M}: {m} {n}")))
        .build();

    // Setup of file logging tools
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%+)(utc)} [{f}:{L}] {h({l})} ! BACKUP_LOGGER ! {M}: {m} {n}")))
        .build("imbriqua_engine.log")?;

    // Setup of global logger
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().appender("stdout").build("app::stdout", LevelFilter::Debug))
        .logger(Logger::builder().appender("requests").build("app::requests", LevelFilter::Debug))
        .build(Root::builder().appender("stdout").appender("requests").build(LevelFilter::Debug))?;

    Ok(config)
}