use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::game::*;

pub const ROWS: usize = 4;
pub const COLS: usize = 3;
pub const BOARD_SIZE: usize = ROWS * COLS;

pub type Row = usize;
pub type Col = usize;
pub type Field = usize;

pub fn rowcol2field(row: Row, col: Col) -> Field {
    COLS * row + col
}

pub const ALL_INDEXES: [Field; BOARD_SIZE] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

pub struct Board(pub [Option<Piece>; BOARD_SIZE]);

impl Board {
    pub fn empty() -> Self {
        let board = [None; BOARD_SIZE];
        Board(board)
    }
}

impl Index<usize> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

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
    use super::rowcol2field;

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
            .map(|(row, col)| rowcol2field(*row, *col))
            .collect();
        assert_eq!(results, (0..12).collect::<Vec<usize>>())
    }
}
