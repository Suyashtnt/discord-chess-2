mod prisma;
use anyhow::{anyhow, Context};
use once_cell::sync::OnceCell;
pub use prisma::*;
use tracing::info;

pub static DATABASE: OnceCell<PrismaClient> = OnceCell::new();

pub async fn entrypoint() -> anyhow::Result<()> {
    info!("initalizing database...");

    DATABASE
        .set(
            new_client()
                .await
                .context("Failed to initialize database")?,
        )
        .map_err(|_e| {
            anyhow!(
            "Failed to set DATABASE instance. This means that it has already been defined before",
        )
        })?;

    info!("Successfully initialised database");

    Ok(())
}
