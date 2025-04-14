mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::server().await?;
    Ok(())
}
