use chrono::{DateTime, FixedOffset};
use core::fmt;
use roxmltree::Document;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Source {
    #[default]
    RSS,
    Youtube,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let res = match &self {
            Self::RSS => "RSS",
            Self::Youtube => "Youtube",
        };
        write!(f, "{}", res)
    }
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        match &value[..] {
            "rss" => Self::RSS,
            "youtube" => Self::Youtube,
            _ => unimplemented!("Invalid command"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Category {
    #[default]
    Entertainment,
    Music,
    Technology,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let res = match &self {
            Self::Entertainment => "Entertainment",
            Self::Music => "Music",
            Self::Technology => "Technology",
        };
        write!(f, "{}", res)
    }
}

impl From<String> for Category {
    fn from(value: String) -> Self {
        match &value[..] {
            "entertainment" => Self::Entertainment,
            "music" => Self::Music,
            "technology" => Self::Technology,
            _ => unimplemented!("Invalid command"),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Feed {
    pub id: String,
    pub source: Source,
    pub category: Category,
    pub url: Option<String>,
}

#[derive(Default, Debug)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub link: String,
    pub published: Option<DateTime<FixedOffset>>,
}

impl Feed {
    async fn get_url(&self) -> Option<String> {
        if let Some(url) = &self.url {
            return Some(url.to_string());
        }
        match self.source {
            Source::Youtube => {
                let channel_url = format!("https://www.youtube.com/{}", self.id);
                if let Ok(yt_channel) = reqwest::get(channel_url).await {
                    if let Ok(yt_channel) = yt_channel.text().await {
                        let html = Html::parse_document(&yt_channel);
                        if let Ok(selector) = Selector::parse("link[title=RSS]") {
                            let element = html.select(&selector).next().unwrap();
                            return Some(element.value().attr("href").unwrap().to_string());
                        }
                    }
                }
                None
            }
            _ => unimplemented!(),
        }
    }

    pub async fn get_entries(&self) -> Vec<Entry> {
        if let Some(url) = &self.get_url().await {
            if let Ok(res) = reqwest::get(url).await {
                if let Ok(content) = res.text().await {
                    return Document::parse(&content).map_or(vec![], |document| {
                        document
                            .descendants()
                            .filter(|des| des.tag_name().name() == "entry")
                            .map(|entry| entry.descendants())
                            .map(|descendants| {
                                let mut feed_entry = Entry::default();
                                descendants.for_each(|des| {
                                    match des.tag_name().name() {
                                        "id" => {
                                            feed_entry.id =
                                                des.text().map_or("".into(), |t| t.to_string())
                                        }
                                        "title" => {
                                            feed_entry.title =
                                                des.text().map_or("".into(), |t| t.to_string())
                                        }
                                        "published" => {
                                            let published =
                                                des.text().map_or(String::new(), |t| t.to_string());
                                            feed_entry.published = if published == *"" {
                                                None
                                            } else {
                                                DateTime::parse_from_rfc3339(&published).ok()
                                            };
                                        }
                                        "link" => {
                                            feed_entry.link = des
                                                .attribute("href")
                                                .map_or(String::new(), |a| a.to_string())
                                        }
                                        _ => (),
                                    };
                                });

                                feed_entry
                            })
                            .collect()
                    });
                }
            }
        }
        vec![]
    }
}
