use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::bail;
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = "config.json";
const CONFIG_DIR: &str = ".config";
const APP_NAME: &str = "dgg-tui";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub name: String,
    pub path: Option<PathBuf>,
}

impl Config {
    pub fn get_or_build_paths(&mut self) -> Result<(), anyhow::Error> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_NAME);

                if !home_config_dir.exists() {
                    fs::create_dir(&home_config_dir)?;
                }

                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(FILE_NAME);

                self.path = Some(config_file_path.to_path_buf());

                println!("{:?}", self.path);

                Ok(())
            }
            None => bail!("No HOME direction found."),
        }
    }

    pub fn read_user_data_from_file(&self) -> anyhow::Result<Config> {
        let file = File::open(self.path.as_ref().unwrap())?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn askers(&mut self) -> anyhow::Result<()> {
        self.name = String::from("onlyclose");
        self.token = String::from("token");
        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        std::fs::write(
            self.path.as_ref().unwrap(),
            serde_json::to_string_pretty(&self).unwrap(),
        )?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            token: String::from(""),
            name: String::from(""),
            path: None,
        }
    }
}
