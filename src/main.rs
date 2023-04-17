mod xml;
mod youtube;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};

use youtube::Youtube;

type MimResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Serialize, Deserialize, Default)]
enum FeedSource {
    #[default]
    RSS,
    Youtube,
}

#[derive(Serialize, Deserialize, Default)]
struct Mim {
    categories: Vec<MimCategory>,
}

#[derive(Serialize, Deserialize, Default)]
struct MimCategory {
    name: String,
    sources: Vec<MimSource>,
}

#[derive(Serialize, Deserialize, Default)]
struct MimSource {
    source: FeedSource,
    value: String,
}

#[derive(Debug)]
pub struct FeedEntry {
    source: FeedSource,
    id: String,
    title: String,
    link: String,
    published: Option<DateTime<FixedOffset>>,
    thumbnail: Option<String>,
}

impl Mim {
    fn load() -> MimResult<Self> {
        if let Some(home_dir) = dirs::home_dir() {
            if let Some(h) = home_dir.to_str() {
                let config_file = PathBuf::from(format!("{}/.mim", h));
                if let Ok(config) = std::fs::read(config_file) {
                    let config = String::from_utf8(config)?;
                    let config: Mim = ron::from_str(&config[..])?;
                    return Ok(config);
                }
            }
        }
        Ok(Mim::default())
    }

    fn save(&self) -> MimResult<()> {
        if let Some(home_dir) = dirs::home_dir() {
            if let Some(h) = home_dir.to_str() {
                let config_file = PathBuf::from(format!("{}/.mim", h));
                std::fs::write(config_file, ron::to_string(&self)?)?;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> MimResult<()> {
    let mim = Mim::load()?;
    let mim_cats = mim.categories.iter();
    for cat in mim_cats {
        println!("{}", cat.name);
        let cat_sources = cat.sources.iter();
        for source in cat_sources {
            match source.source {
                FeedSource::RSS => {
                    todo!("Standard RSS feed not implemented")
                }
                FeedSource::Youtube => {
                    let rss_url = Youtube::get_rss_url(source.value.clone()).await?;
                    let entries = Youtube::get_rss_entries(rss_url).await?;
                    for entry in entries {
                        println!(
                            "{:?} | {:?} | {:?} | {:?} | {:?} | {:?}",
                            entry.source,
                            entry.id,
                            entry.title,
                            entry.published,
                            entry.link,
                            entry.thumbnail
                        );
                    }
                }
            }
        }
    }
    mim.save()
}
