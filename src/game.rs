/// A structured representation of the game
///
/// It's not very efficient, but it encodes many invariants in the code.
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum PieceKind {
    Pawn,
    Bishop,
    Rook,
    King,
    PromotedPawn,
}
#[derive(Debug, Copy, Clone)]
pub enum HandPiece {
    Pawn,
    Bishop,
    Rook,
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

pub const ROWS: usize = 4;
pub const COLS: usize = 3;
pub const BOARD_SIZE: usize = ROWS * COLS;

pub fn rowcol2index(row: usize, col: usize) -> usize {
    COLS * row + col
}

pub struct Board(pub [Option<Piece>; BOARD_SIZE]);

pub struct Hand(pub HashMap<HandPiece, usize>);

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

#[cfg(test)]
mod tests {
    use super::rowcol2index;

    #[test]
    pub fn test_rowcol_conversion() {
        let points: [(usize, usize); 12] = [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
            (3, 0),
            (3, 1),
            (3, 2),
        ];
        let results: Vec<usize> = points
            .iter()
            .map(|(row, col)| rowcol2index(*row, *col))
            .collect();
        assert_eq!(results, (0..12).collect::<Vec<usize>>())
    }
}
