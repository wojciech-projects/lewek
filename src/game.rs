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
