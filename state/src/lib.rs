use std::{
    fmt::{self, Display},
    sync::Mutex,
};

use error_stack::{Context, IntoReport, Result, ResultExt};
use once_cell::sync::OnceCell;

/// A currently active game
pub struct Game {
    /// A list of moves in standard algebreic notation
    pub moves: Vec<String>,
    /// The current board in FEN
    pub board: String,
    /// White's discord UID
    pub white: String,
    /// Blacks's discord UID
    pub black: String,
}

pub static GAMES: OnceCell<Mutex<Vec<Game>>> = OnceCell::new();

#[derive(Debug)]
pub struct StateInitError;

impl Display for StateInitError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to initialize database")
    }
}

impl Context for StateInitError {}

pub fn entrypoint() -> Result<(), StateInitError> {
    GAMES
        .set(Mutex::from(vec![]))
        .map_err(|_| StateInitError)
        .into_report()
        .attach_printable("Could not set GAMES state")?;

    Ok(())
}
