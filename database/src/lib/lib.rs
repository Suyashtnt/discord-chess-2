mod prisma;

use std::fmt::{self, Display};

use error_stack::{report, Context, IntoReport, Result, ResultExt};
use once_cell::sync::OnceCell;
pub use prisma::*;
use tracing::info;

#[derive(Debug)]
pub struct DatabaseInitError;

impl Display for DatabaseInitError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to initialize database")
    }
}

impl Context for DatabaseInitError {}

pub static DATABASE: OnceCell<PrismaClient> = OnceCell::new();

#[tracing::instrument]
pub async fn entrypoint() -> Result<(), DatabaseInitError> {
    info!("initalizing database...");

    DATABASE
        .set(
            new_client()
                .await
                .report()
                .attach_printable("Could not connect to the database!")
                .change_context(DatabaseInitError)?,
        )
        .map_err(|_| report!(DatabaseInitError))
        .attach_printable("Could not set the database instance!")?;

    info!("Successfully initialised database");

    Ok(())
}
