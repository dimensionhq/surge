use std::path::PathBuf;

use crate::{config::Config, external};

/// This struct sets the loggers and the config file up
///
/// ```rust
/// use utils::app::App;
///
/// let app = App::new();
/// ```
#[derive(Debug)]
pub struct App {
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
    pub config: Config,
}

impl App {
    /// creates a new instance of App
    /// and then loads the config file
    pub fn new() -> Self {
        let config_dir = dirs::config_dir().unwrap();
        let config_file = config_dir.join("config.toml");
        external::ansi::enable_ansi_support().unwrap();

        let config = Config {
            host_port: 8000,
            name: String::from("surge"),
            storage_port: 8080,
        };
        App {
            config_dir,
            config_file,
            config,
        }
    }
}
