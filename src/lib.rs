mod feed;

use feed::Feed;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};

pub type MimResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Default)]
pub struct Mim {
    pub feeds: Vec<Feed>,
}

impl Mim {
    fn get_path() -> MimResult<PathBuf> {
        let file_location = dirs::home_dir().expect("Could not get the home directory");
        let file_location = file_location.join(".config").join("atelier");
        std::fs::create_dir_all(&file_location)?;
        Ok(file_location.join("mim"))
    }

    pub fn load() -> MimResult<Self> {
        Self::get_path()
            .ok()
            .and_then(|config_path| std::fs::read(config_path).ok())
            .and_then(|config| String::from_utf8(config).ok())
            .map_or(Ok(Mim::default()), |config| {
                Ok(ron::from_str::<Mim>(&config[..])?)
            })
    }

    pub fn save(&self) -> MimResult<()> {
        if let Ok(config_file) = Self::get_path() {
            if let Ok(ron_string) = ron::to_string(&self) {
                std::fs::write(config_file, ron_string)?;
            }
        }
        Ok(())
    }
}
