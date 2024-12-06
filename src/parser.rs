use crate::game::*;

fn parse_color(color: &str) -> Option<Color> {
    match color {
        "b" => Some(Color::Black),
        "w" => Some(Color::White),
        _ => None,
    }
}

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

fn parse_board(board: &str) -> Option<Board> {
    Some(Board::empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let inputs = [
            ("b", Some(Color::Black)),
            ("w", Some(Color::White)),
            ("", None),
            ("!", None),
        ];

        for (string, expected_result) in inputs {
            let result = parse_color(string);
            assert_eq!(result, expected_result);
        }
    }

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
            assert_eq!(result, Some(expected_piece));
        }
    }

    #[test]
    fn test_parse_piece_incorrect() {
        let inputs = [
            "", "x", "123", "n", "g", "s", // shogi pieces not in Let's catch the Lion
        ];

        for string in inputs {
            let result = parse_piece(string);
            assert_eq!(result, None);
        }
    }

    #[test]
    fn test_parse_empty_board() {
        let input = "3/3/3/3";

        let result_board = parse_board(input).unwrap();

        for index in ALL_INDEXES {
            let piece = result_board[index];
            assert!(piece.is_none());
        }
    }
}
