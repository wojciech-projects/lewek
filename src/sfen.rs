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

impl Sfen for HandPiece {
    fn sfen(&self) -> String {
        match *self {
            HandPiece::Pawn => "p",
            HandPiece::Bishop => "b",
            HandPiece::Rook => "r",
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
                let index = rowcol2field(row, col);
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

impl Sfen for Hand {
    fn sfen(&self) -> String {
        let mut result = String::new();

        for piece in ALL_HAND_PIECES {
            if let Some(&count) = self.0.get(&piece) {
                if count > 1 {
                    result += &format!("{}{}", piece.sfen(), count);
                } else if count == 1 {
                    result += &piece.sfen();
                }
            }
        }
        return result;
    }
}

impl Sfen for Position {
    fn sfen(&self) -> String {
        let mut hands_sfen = String::new();
        hands_sfen += &self.black_hand.sfen().to_ascii_uppercase();
        hands_sfen += &self.white_hand.sfen();

        format!(
            "{} {} {}",
            self.board.sfen(),
            self.to_play.sfen(),
            if hands_sfen == "" { "-" } else { &hands_sfen }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

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
    pub fn test_handpiece() {
        let pieces = Vec::from(ALL_HAND_PIECES);
        let result: Vec<String> = pieces.iter().map(|piece| piece.sfen()).collect();

        assert_eq!(result, vec!["p", "b", "r"],);
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
        let board = Board([
            Some(WHITE_PAWN),
            None,
            Some(WHITE_KING),
            Some(WHITE_BISHOP),
            None,
            None,
            None,
            None,
            Some(BLACK_ROOK),
            None,
            Some(BLACK_PROMOTED_PAWN),
            None,
        ]);
        let result = board.sfen();

        assert_eq!(result, "p1k/b2/2R/1P+1");
    }

    #[test]
    pub fn test_empty_hand() {
        let hand = Hand(HashMap::new());
        let result = hand.sfen();

        assert_eq!(result, "");
    }

    #[test]
    pub fn test_hand_all_one() {
        let hashmap = HashMap::from([
            (HandPiece::Bishop, 1),
            (HandPiece::Rook, 1),
            (HandPiece::Pawn, 1),
        ]);
        let hand = Hand(hashmap);
        let result = hand.sfen();

        assert_eq!(result, "pbr");
    }

    #[test]
    pub fn test_hand_all_two() {
        let hashmap = HashMap::from([
            (HandPiece::Bishop, 2),
            (HandPiece::Rook, 2),
            (HandPiece::Pawn, 2),
        ]);
        let hand = Hand(hashmap);
        let result = hand.sfen();

        assert_eq!(result, "p2b2r2");
    }

    #[test]
    pub fn test_hand_mixed() {
        let hashmap = HashMap::from([
            (HandPiece::Bishop, 0),
            (HandPiece::Rook, 1),
            (HandPiece::Pawn, 2),
        ]);
        let hand = Hand(hashmap);
        let result = hand.sfen();

        assert_eq!(result, "p2r");
    }

    #[test]
    pub fn test_position_starting() {
        let board = Board([
            Some(WHITE_ROOK),
            Some(WHITE_KING),
            Some(WHITE_BISHOP),
            //-----
            None,
            Some(WHITE_PAWN),
            None,
            //-----
            None,
            Some(BLACK_PAWN),
            None,
            // ----
            Some(BLACK_BISHOP),
            Some(BLACK_KING),
            Some(BLACK_ROOK),
        ]);

        let position = Position {
            board,
            to_play: Color::Black,
            black_hand: Hand(HashMap::new()),
            white_hand: Hand(HashMap::new()),
        };

        let result = position.sfen();

        assert_eq!(result, "rkb/1p1/1P1/BKR b -");
    }

    #[test]
    pub fn test_position_with_hands() {
        let board = Board([
            None,
            Some(WHITE_KING),
            None,
            //-----
            None,
            None,
            None,
            //-----
            None,
            None,
            None,
            // ----
            None,
            Some(BLACK_KING),
            Some(BLACK_ROOK),
        ]);

        let black_hashmap = HashMap::from([(HandPiece::Rook, 1), (HandPiece::Pawn, 2)]);
        let black_hand = Hand(black_hashmap);

        let white_hashmap = HashMap::from([(HandPiece::Bishop, 2)]);
        let white_hand = Hand(white_hashmap);

        let position = Position {
            board,
            to_play: Color::White,
            black_hand,
            white_hand,
        };

        let result = position.sfen();

        assert_eq!(result, "1k1/3/3/1KR w P2Rb2");
    }
}
