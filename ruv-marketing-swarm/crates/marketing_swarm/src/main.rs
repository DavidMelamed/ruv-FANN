#[tokio::main]
async fn main() -> anyhow::Result<()> {
    marketing_swarm::serve().await
}
