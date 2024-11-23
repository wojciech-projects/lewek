/// A structured representation of the game
///
/// It's not very efficient, but it encodes many invariants in the code.
use std::collections::HashMap;

#[derive(Debug)]
pub enum PieceKind {
    Pawn,
    Bishop,
    Rook,
    King,
    PromotedPawn,
}

pub enum HandPiece {
    Pawn,
    Bishop,
    Rook,
}

pub enum Color {
    Black,
    White,
}

pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

pub const BOARD_SIZE: usize = 12;

pub type Board = [Piece; BOARD_SIZE];

pub type Hand = HashMap<HandPiece, usize>;

pub struct Position {
    pub board: Board,
    pub to_play: Color,
    pub black_hand: Hand,
    pub white_hand: Hand,
}

/// In order to be able to detect draws by repetition
/// we hold a history of positions.
///
/// This can be optimized by keeping hashes or sfens instead.
pub struct GameState {
    pub current_position: Position,
    pub previous_positions: Vec<Position>,
}

trait Sfen {
    fn sfen(&self) -> String;
}

impl Sfen for PieceKind {
    fn sfen(&self) -> String {
        match *self {
            PieceKind::Pawn => "p",
            PieceKind::Bishop => "b",
            PieceKind::Rook => "r",
            PieceKind::King => "k",
            PieceKind::PromotedPawn => "p+",
        }
        .to_owned()
    }
}

impl Sfen for Color {
    fn sfen(&self) -> String {
        match *self {
            Color::Black => "b",
            Color::White => "w",
        }
        .to_owned()
    }
}

impl Sfen for Piece {
    fn sfen(&self) -> String {
        let Piece { kind, color } = self;

        let piecekind_sfen = kind.sfen();

        match color {
            Color::Black => piecekind_sfen.to_ascii_uppercase(),
            Color::White => piecekind_sfen,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_PIECES: [Piece; 10] = [
        Piece {
            kind: PieceKind::Pawn,
            color: Color::Black,
        },
        Piece {
            kind: PieceKind::Bishop,
            color: Color::Black,
        },
        Piece {
            kind: PieceKind::Rook,
            color: Color::Black,
        },
        Piece {
            kind: PieceKind::King,
            color: Color::Black,
        },
        Piece {
            kind: PieceKind::PromotedPawn,
            color: Color::Black,
        },
        Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        },
        Piece {
            kind: PieceKind::Bishop,
            color: Color::White,
        },
        Piece {
            kind: PieceKind::Rook,
            color: Color::White,
        },
        Piece {
            kind: PieceKind::King,
            color: Color::White,
        },
        Piece {
            kind: PieceKind::PromotedPawn,
            color: Color::White,
        },
    ];

    #[test]
    pub fn test_piece_kind() {
        use PieceKind::*;
        let pieces = vec![Pawn, Bishop, Rook, King, PromotedPawn];
        let result: Vec<String> = pieces.iter().map(|piece| piece.sfen()).collect();

        assert_eq!(result, vec!["p", "b", "r", "k", "p+"]);
    }

    #[test]
    pub fn test_color() {
        let colors = vec![Color::Black, Color::White];
        let result: Vec<String> = colors.iter().map(|color| color.sfen()).collect();

        assert_eq!(result, vec!["b", "w"]);
    }

    #[test]
    pub fn test_piece() {
        let pieces = Vec::from(ALL_PIECES);
        let result: Vec<String> = pieces.iter().map(|piece| piece.sfen()).collect();

        assert_eq!(
            result,
            vec!["P", "B", "R", "K", "P+", "p", "b", "r", "k", "p+"],
        );
    }
}
