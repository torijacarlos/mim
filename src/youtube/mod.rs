use crate::xml::NodeManipulation;
use crate::{FeedEntry, FeedSource, MimResult};
use chrono::DateTime;
use roxmltree::Document;
use scraper::{Html, Selector};

pub struct Youtube;

impl Youtube {
    pub async fn get_rss_url(handler: String) -> MimResult<String> {
        let yt_channel = reqwest::get(format!("https://www.youtube.com/{handler}"))
            .await?
            .text()
            .await?;
        let html = Html::parse_document(&yt_channel);
        let selector = Selector::parse("link[title=RSS]")?;
        let element = html.select(&selector).next().unwrap();

        Ok(element.value().attr("href").unwrap().to_string())
    }

    pub async fn get_rss_entries(rss_url: String) -> MimResult<Vec<FeedEntry>> {
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
                    source: FeedSource::Youtube,
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
