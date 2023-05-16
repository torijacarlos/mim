#[tokio::main]
async fn main() -> mim::MimResult<()> {
    let mim = mim::Mim::load()?;
    for feed in mim.feeds.iter() {
        println!("{} {} {}", feed.source, feed.category, feed.value);
        for entry in feed.get_entries().await {
            println!("{:?}", entry,);
        }
    }
    mim.save()
}
