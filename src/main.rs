use chrono::{DateTime, FixedOffset};
use roxmltree::Document;
use scraper::{Html, Selector};
use std::error::Error;

struct Youtube;

impl Youtube {
    async fn get_rss_url(handler: String) -> Result<String, Box<dyn Error>> {
        let yt_channel = reqwest::get(format!("https://www.youtube.com/{handler}"))
            .await?
            .text()
            .await?;
        let html = Html::parse_document(&yt_channel);
        let selector = Selector::parse("link[title=RSS]")?;
        let element = html.select(&selector).next().unwrap();

        Ok(element.value().attr("href").unwrap().to_string())
    }

    async fn get_rss_entries(rss_url: String) -> Result<Vec<FeedEntry>, Box<dyn Error>> {
        let content = reqwest::get(&rss_url).await?.text().await?;
        let rss = Document::parse(&content)?;
        let entries = rss
            .descendants()
            .filter(|des| des.tag_name().name() == "entry")
            .map(|entry| {
                let mut descendants = entry.descendants();
                let id = descendants
                    .find(|des| des.tag_name().name() == "id")
                    .unwrap()
                    .text()
                    .unwrap()
                    .to_string();
                let title = descendants
                    .find(|des| des.tag_name().name() == "title")
                    .unwrap()
                    .text()
                    .unwrap()
                    .to_string();
                let link = descendants
                    .find(|des| des.tag_name().name() == "link")
                    .unwrap()
                    .attribute("href")
                    .unwrap()
                    .to_string();
                let published = descendants
                    .find(|des| des.tag_name().name() == "published")
                    .unwrap()
                    .text()
                    .unwrap()
                    .to_string();
                let media = descendants
                    .find(|des| des.tag_name().name() == "group")
                    .unwrap();
                let thumbnail = media
                    .descendants()
                    .find(|des| des.tag_name().name() == "thumbnail")
                    .unwrap()
                    .attribute("url")
                    .unwrap()
                    .to_string();
                let published = DateTime::parse_from_rfc3339(&published).ok();
                FeedEntry {
                    source: EntrySource::Youtube,
                    id,
                    title,
                    link,
                    published,
                    thumbnail: Some(thumbnail),
                }
            })
            .collect();
        Ok(entries)
    }
}

#[derive(Debug)]
enum EntrySource {
    Youtube,
}

#[derive(Debug)]
struct FeedEntry {
    source: EntrySource,
    id: String,
    title: String,
    link: String,
    published: Option<DateTime<FixedOffset>>,
    thumbnail: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rss_url = Youtube::get_rss_url("@ConnorDawg".to_string()).await?;
    let entries = Youtube::get_rss_entries(rss_url).await?;
    for entry in entries {
        println!(
            "{:?} | {:?} | {:?} | {:?} | {:?} | {:?}",
            entry.source, entry.id, entry.title, entry.published, entry.link, entry.thumbnail
        );
    }
    Ok(())
}
