use crate::game::*;

fn parse_piece(piece: &str) -> Option<Piece> {
    match piece {
        "p" => Some(WHITE_PAWN),
        "b" => Some(WHITE_BISHOP),
        "r" => Some(WHITE_ROOK),
        "k" => Some(WHITE_KING),
        "p+" => Some(WHITE_PROMOTED_PAWN),
        "P" => Some(BLACK_PAWN),
        "B" => Some(BLACK_BISHOP),
        "R" => Some(BLACK_ROOK),
        "K" => Some(BLACK_KING),
        "P+" => Some(BLACK_PROMOTED_PAWN),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_piece_correct() {
        let inputs = [
            ("p", WHITE_PAWN),
            ("b", WHITE_BISHOP),
            ("r", WHITE_ROOK),
            ("k", WHITE_KING),
            ("p+", WHITE_PROMOTED_PAWN),
            ("P", BLACK_PAWN),
            ("B", BLACK_BISHOP),
            ("R", BLACK_ROOK),
            ("K", BLACK_KING),
            ("P+", BLACK_PROMOTED_PAWN),
        ];

        for (string, expected_piece) in inputs {
            let result = parse_piece(string);
        let inputs = [
        }
    }
}
