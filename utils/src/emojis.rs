use std::fmt::{Display, Error, Formatter};

use shakmaty::Piece;

pub enum PieceEmoji {
    // EMPTY SQUARES
    DarkEmpty,
    LightEmpty,

    // WHITE PIECES
    // LIGHT
    LightWhitePawn,
    LightWhiteBishop,
    LightWhiteKnight,
    LightWhiteRook,
    LightWhiteQueen,
    LightWhiteKing,

    // DARK
    DarkWhitePawn,
    DarkWhiteBishop,
    DarkWhiteKnight,
    DarkWhiteRook,
    DarkWhiteQueen,
    DarkWhiteKing,

    // BLACK PIECE
    // LIGHT
    LightBlackPawn,
    LightBlackBishop,
    LightBlackKnight,
    LightBlackRook,
    LightBlackQueen,
    LightBlackKing,

    // DARK
    DarkBlackPawn,
    DarkBlackBishop,
    DarkBlackKnight,
    DarkBlackRook,
    DarkBlackQueen,
    DarkBlackKing,
}

impl PieceEmoji {
    pub fn from_piece(piece: Piece, is_light: bool) -> Self {
        match piece.role {
            shakmaty::Role::Pawn => match piece.color {
                shakmaty::Color::Black => {
                    if is_light {
                        Self::LightBlackPawn
                    } else {
                        Self::DarkBlackPawn
                    }
                }
                shakmaty::Color::White => {
                    if is_light {
                        Self::LightWhitePawn
                    } else {
                        Self::DarkWhitePawn
                    }
                }
            },

            shakmaty::Role::Knight => match piece.color {
                shakmaty::Color::Black => {
                    if is_light {
                        Self::LightBlackKnight
                    } else {
                        Self::DarkBlackKnight
                    }
                }
                shakmaty::Color::White => {
                    if is_light {
                        Self::LightWhiteKnight
                    } else {
                        Self::DarkWhiteKnight
                    }
                }
            },

            shakmaty::Role::Bishop => match piece.color {
                shakmaty::Color::Black => {
                    if is_light {
                        Self::LightBlackBishop
                    } else {
                        Self::DarkBlackBishop
                    }
                }
                shakmaty::Color::White => {
                    if is_light {
                        Self::LightWhiteBishop
                    } else {
                        Self::DarkWhiteBishop
                    }
                }
            },

            shakmaty::Role::Rook => match piece.color {
                shakmaty::Color::Black => {
                    if is_light {
                        Self::LightBlackRook
                    } else {
                        Self::DarkBlackRook
                    }
                }
                shakmaty::Color::White => {
                    if is_light {
                        Self::LightWhiteRook
                    } else {
                        Self::DarkWhiteRook
                    }
                }
            },

            shakmaty::Role::Queen => match piece.color {
                shakmaty::Color::Black => {
                    if is_light {
                        Self::LightBlackQueen
                    } else {
                        Self::DarkBlackQueen
                    }
                }
                shakmaty::Color::White => {
                    if is_light {
                        Self::LightWhiteQueen
                    } else {
                        Self::DarkWhiteQueen
                    }
                }
            },

            shakmaty::Role::King => match piece.color {
                shakmaty::Color::Black => {
                    if is_light {
                        Self::LightBlackKing
                    } else {
                        Self::DarkBlackKing
                    }
                }
                shakmaty::Color::White => {
                    if is_light {
                        Self::LightWhiteKing
                    } else {
                        Self::DarkWhiteKing
                    }
                }
            },
        }
    }

    pub fn into_emoji_str(&self) -> String {
        format!("<:pieceEmoji:{}>", self)
    }
}

impl Display for PieceEmoji {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use PieceEmoji::*;
        write!(
            f,
            "{}",
            match self {
                // EMPTY SQUARES
                DarkEmpty => "979399119397355520",
                LightEmpty => "979399119644799026",

                // WHITE PIECES
                // LIGHT
                LightWhitePawn => "979396455162863676",
                LightWhiteBishop => "979396454944751656",
                LightWhiteKnight => "979396455196426280",
                LightWhiteRook => "979396454986702848",
                LightWhiteQueen => "979396455112532019",
                LightWhiteKing => "979396455162871878",

                // DARK
                DarkWhitePawn => "979396455192223824",
                DarkWhiteBishop => "979396455280304199",
                DarkWhiteKnight => "979397693493346344",
                DarkWhiteRook => "979397692914557012",
                DarkWhiteQueen => "979442406992797716",
                DarkWhiteKing => "979397693375926272",

                // BLACK PIECE
                // LIGHT
                LightBlackPawn => "979396455255138304",
                LightBlackBishop => "979396455129313370",
                LightBlackKnight => "979396455095750686",
                LightBlackRook => "979396454965731369",
                LightBlackQueen => "979442406992801822",
                LightBlackKing => "979396454726647819",

                // DARK
                DarkBlackPawn => "979396455498416158",
                DarkBlackBishop => "979396454986711040",
                DarkBlackKnight => "979397693619187733",
                DarkBlackRook => "979397693279453255",
                DarkBlackQueen => "979396454957338634",
                DarkBlackKing => "979396454886035477",
            }
        )
    }
}
