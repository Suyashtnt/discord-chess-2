#[tokio::main]
async fn main() -> anyhow::Result<()> {
    database::entrypoint().await?;
    Ok(())
}
