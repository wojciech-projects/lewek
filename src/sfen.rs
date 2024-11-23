use crate::game::*;

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

impl Sfen for Board {
    fn sfen(&self) -> String {
        fn render_row(board: &Board, row: usize) -> String {
            let mut line = String::new();
            let mut empty_count = 0;
            for col in 0..COLS {
                let index = rowcol2index(row, col);
                if let Some(piece) = board.0[index] {
                    if empty_count > 0 {
                        line += &empty_count.to_string();
                    }
                    line += &piece.sfen();
                    empty_count = 0;
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                line += &empty_count.to_string();
            }
            line
        }

        let mut lines = vec![];
        for row in 0..ROWS {
            lines.push(render_row(self, row));
        }
        lines.join("/")
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

    #[test]
    pub fn test_empty_board() {
        let board = Board([None; BOARD_SIZE]);
        let result = board.sfen();

        assert_eq!(result, "3/3/3/3");
    }

    #[test]
    pub fn test_board_of_white_pawns() {
        let piece = Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        };
        let board = Board([Some(piece); BOARD_SIZE]);
        let result = board.sfen();

        assert_eq!(result, "ppp/ppp/ppp/ppp");
    }

    #[test]
    pub fn test_board_with_many_gaps_and_pieces() {
        let white_pawn = Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        };
        let white_king = Piece {
            kind: PieceKind::King,
            color: Color::White,
        };
        let white_bishop = Piece {
            kind: PieceKind::Bishop,
            color: Color::White,
        };
        let black_rook = Piece {
            kind: PieceKind::Rook,
            color: Color::Black,
        };
        let black_tokin = Piece {
            kind: PieceKind::PromotedPawn,
            color: Color::Black,
        };
        let board = Board([
            Some(white_pawn),
            None,
            Some(white_king),
            Some(white_bishop),
            None,
            None,
            None,
            None,
            Some(black_rook),
            None,
            Some(black_tokin),
            None,
        ]);
        let result = board.sfen();

        assert_eq!(result, "p1k/b2/2R/1P+1");
    }
}
