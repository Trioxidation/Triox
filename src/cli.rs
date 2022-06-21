use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub struct Options {
    pub config_dir: String,
    pub log_level: String,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            config_dir: String::from("config"),
            log_level: String::from("info"),
        }
    }
}

impl Options {
    pub fn new() -> Self {
        let mut options = Options::default();

        let matches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::with_name("config-dir")
                    .short('c')
                    .long("config-dir")
                    .takes_value(true)
                    .help("Path to configuration file directory."),
            )
            .arg(
                Arg::with_name("default-log-level")
                    .long("default-log-level")
                    .takes_value(true)
                    .help("Set default log level."),
            )
            .get_matches();

        if let Some(config_dir) = matches.value_of("config-dir") {
            options.config_dir = config_dir.to_owned();
        }

        if let Some(log_level) = matches.value_of("default-log-level") {
            match log_level {
                "warn" | "trace" | "debug" | "error" | "info" => {
                    options.log_level = log_level.to_owned()
                }
                _ => eprintln!(
                    "Incorrect value for default-log-level, using default value instead"
                ),
            }
        }

        options
    }
}
