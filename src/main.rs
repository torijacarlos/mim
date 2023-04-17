use chrono::{DateTime, FixedOffset};
use roxmltree::{Document, Node};
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
                let id = NodeManipulation::get_text_from_node(
                    descendants.find(|des| des.tag_name().name() == "id"),
                );
                let title = NodeManipulation::get_text_from_node(
                    descendants.find(|des| des.tag_name().name() == "title"),
                );
                let link = NodeManipulation::get_attr_from_node(
                    descendants.find(|des| des.tag_name().name() == "link"),
                    "href".to_string(),
                );
                let published = NodeManipulation::get_text_from_node(
                    descendants.find(|des| des.tag_name().name() == "published"),
                );
                let media = descendants
                    .find(|des| des.tag_name().name() == "group")
                    .unwrap();
                let thumbnail = NodeManipulation::get_attr_from_node(
                    media
                        .descendants()
                        .find(|des| des.tag_name().name() == "thumbnail"),
                    "url".to_string(),
                );
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

struct NodeManipulation;

impl NodeManipulation {
    #[inline]
    fn get_text_from_node(node: Option<Node>) -> String {
        if let Some(n) = node {
            if let Some(t) = n.text() {
                return t.to_string();
            }
        }
        String::new()
    }

    #[inline]
    fn get_attr_from_node(node: Option<Node>, attr: String) -> String {
        if let Some(n) = node {
            if let Some(a) = n.attribute(&attr[..]) {
                return a.to_string();
            }
        }
        String::new()
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
