pub mod emojis;
use emojis::*;
use shakmaty::{Board, Square};

pub fn create_emoji_board(board: Board) -> String {
    let mut board_str = String::new();

    for sq in Square::ALL {
        if let Some(piece) = board.piece_at(sq) {
            board_str.push_str(&PieceEmoji::from_piece(piece, sq.is_light()).into_emoji_str())
        } else {
            if sq.is_light() {
                board_str.push_str(&PieceEmoji::LightEmpty.into_emoji_str())
            } else {
                board_str.push_str(&PieceEmoji::DarkEmpty.into_emoji_str())
            }
        }
    }

    board_str
}
