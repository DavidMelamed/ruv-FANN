use marketing_divergent::agents::{SeoCrawler, TrendHijacker};
use marketing_divergent::MarketingAgent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let crawler = SeoCrawler;
    let plan = crawler.plan("seed keyword").await?;
    let output = crawler.act(&plan).await?;
    let hijacker = TrendHijacker;
    let plan = hijacker.plan(&output).await?;
    hijacker.act(&plan).await?;
    Ok(())
}
