use std::{
    fs::{self, File},
    io::{self, BufReader},
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
    #[serde(skip_serializing, skip_deserializing)]
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

                Ok(())
            }
            None => bail!("No HOME direction found."),
        }
    }

    pub fn askers(&mut self) -> anyhow::Result<()> {
        if self.token.len() == 64 {
            return Ok(());
        }
        println!(
            "Creating a config file at {}",
            self.path.as_ref().unwrap().display()
        );
        println!("Please get your login token at https://www.destiny.gg/profile/developer");
        println!("Go to Connections and press the 'Add login key' Button.");
        println!("Please paste the DGG Login Key into this console and press enter.");
        println!("");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                self.token = input.trim().to_string();
            }
            Err(_) => (),
        }

        while self.token.len() != 64 {
            println!("");
            println!("There is something wrong with your token!");
            println!("Please make sure you properly pasted it into the console.");
            println!("Please paste it in again and confirm with enter.");

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    self.token = input.trim().to_string();
                }
                Err(_) => (),
            }
        }
        println!("");
        println!("Successfully saved the token!");

        println!("Please write your dgg username and confirm with enter.");
        println!("");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                self.name = input.trim().to_string();
            }
            Err(_) => (),
        }

        Ok(())
    }

    pub fn read_user_data_from_file(&mut self) -> anyhow::Result<()> {
        let file = File::open(self.path.as_ref().unwrap());

        if file.is_err() {
            bail!("No config file exists.");
        }

        let reader = BufReader::new(file.unwrap());
        let config: Config = serde_json::from_reader(reader)?;
        *self = config;

        Ok(())
    }

    pub fn save_to_config_file(&self) -> anyhow::Result<()> {
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
