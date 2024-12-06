use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq)]
enum RowLexerOutput {
    Digit(usize),
    Piece(Piece),
}

fn lex_row(row: &str) -> Vec<RowLexerOutput> {
    let mut result = vec![];
    let chars: Vec<char> = row.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];

        if ch == '+' {
            i += 1;
            continue;
        }

        if let Some(digit) = ch.to_digit(10) {
            result.push(RowLexerOutput::Digit(digit as usize));
            i += 1;
        } else {
            let piece = if i + 1 < chars.len() && chars[i + 1] == '+' {
                String::from(ch) + &String::from(chars[i + 1])
            } else {
                String::from(ch)
            };

            if let Some(piece) = parse_piece(&piece) {
                result.push(RowLexerOutput::Piece(piece));
            }
            i += 1
        }
    }

    result
}

fn parse_board(board: &str) -> Option<Board> {
    let rows: Vec<&str> = board.split("/").collect();
    if rows.len() != 4 {
        return None;
    }

    let mut board = Board::empty();
    let mut current_field = 0;

    for row in rows {
        for token in lex_row(row) {
            match token {
                RowLexerOutput::Digit(digit) => {
                    current_field += digit;
                }
                RowLexerOutput::Piece(piece) => {
                    board[current_field] = Some(piece);
                    current_field += 1;
                }
            }
        }
    }

    Some(board)
}

fn parse_hand_piece(piece: &str) -> Option<HandPiece> {
    match piece {
        "p" => Some(HandPiece::Pawn),
        "b" => Some(HandPiece::Bishop),
        "r" => Some(HandPiece::Rook),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HandLexerOutput {
    piece: HandPiece,
    color: Color,
    count: usize,
}

fn lex_hand(hands: &str) -> Option<Vec<HandLexerOutput>> {
    let mut result = vec![];

    let chars: Vec<char> = hands.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_digit(10) {
            i += 1;
            continue;
        }
        let mut color = Color::White;
        if chars[i].is_uppercase() {
            color = Color::Black;
        }
        let string = String::from(chars[i]).to_ascii_lowercase();
        let piece = parse_hand_piece(&string)?;

        let mut count = 1;

        if i + 1 < chars.len() {
            if let Some(digit) = chars[i + 1].to_digit(10) {
                count = digit as usize;
            }
        }

        result.push(HandLexerOutput {
            piece,
            color,
            count,
        });
        i += 1;
    }

    Some(result)
}

fn parse_hands(hands: &str) -> Option<HashMap<Color, Hand>> {
    let mut white_hand = Hand(HashMap::new());
    let mut black_hand = Hand(HashMap::new());

    for token in lex_hand(hands)? {
        let HandLexerOutput {
            piece,
            color,
            count,
        } = token;

        match color {
            Color::White => {
                let mut entry = white_hand.0.entry(piece).or_insert(0);
                *entry += count;
            }
            Color::Black => {
                let mut entry = black_hand.0.entry(piece).or_insert(0);
                *entry += count;
            }
        }
    }

    Some(HashMap::from([
        (Color::Black, black_hand),
        (Color::White, white_hand),
    ]))
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
    fn test_parse_invalid_board() {
        let input = "3/3/3"; // missing fields

        let result_board = parse_board(input);

        assert!(result_board.is_none());
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

    #[test]
    fn test_parse_starting_position() {
        let input = "bkr/1p1/1P1/RKB";

        let result_board = parse_board(input).unwrap();

        assert_eq!(result_board[0], Some(WHITE_BISHOP));
        assert_eq!(result_board[1], Some(WHITE_KING));
        assert_eq!(result_board[2], Some(WHITE_ROOK));
        assert_eq!(result_board[3], None);
        assert_eq!(result_board[4], Some(WHITE_PAWN));
        assert_eq!(result_board[5], None);
        assert_eq!(result_board[6], None);
        assert_eq!(result_board[7], Some(BLACK_PAWN));
        assert_eq!(result_board[8], None);
        assert_eq!(result_board[9], Some(BLACK_ROOK));
        assert_eq!(result_board[10], Some(BLACK_KING));
        assert_eq!(result_board[11], Some(BLACK_BISHOP));
    }

    #[test]
    fn test_parse_promoted_pieces() {
        let input = "p+2/1p+1/1P+1/2P+";

        let result_board = parse_board(input).unwrap();

        assert_eq!(result_board[0], Some(WHITE_PROMOTED_PAWN));
        assert_eq!(result_board[1], None);
        assert_eq!(result_board[2], None);

        assert_eq!(result_board[3], None);
        assert_eq!(result_board[4], Some(WHITE_PROMOTED_PAWN));
        assert_eq!(result_board[5], None);

        assert_eq!(result_board[6], None);
        assert_eq!(result_board[7], Some(BLACK_PROMOTED_PAWN));
        assert_eq!(result_board[8], None);

        assert_eq!(result_board[9], None);
        assert_eq!(result_board[10], None);
        assert_eq!(result_board[11], Some(BLACK_PROMOTED_PAWN));
    }

    #[test]
    fn test_row_lexer() {
        use RowLexerOutput::*;

        let inputs = [
            (
                "rkb",
                vec![Piece(WHITE_ROOK), Piece(WHITE_KING), Piece(WHITE_BISHOP)],
            ),
            ("3", vec![Digit(3)]),
            ("1p1", vec![Digit(1), Piece(WHITE_PAWN), Digit(1)]),
            (
                "P+p+p+",
                vec![
                    Piece(BLACK_PROMOTED_PAWN),
                    Piece(WHITE_PROMOTED_PAWN),
                    Piece(WHITE_PROMOTED_PAWN),
                ],
            ),
        ];

        for (string, expected_vec) in inputs {
            let result = lex_row(string);
            assert_eq!(result, expected_vec);
        }
    }

    #[test]
    fn test_parse_hand_piece_correct() {
        let inputs = [
            ("p", HandPiece::Pawn),
            ("b", HandPiece::Bishop),
            ("r", HandPiece::Rook),
        ];

        for (string, expected_piece) in inputs {
            let result = parse_hand_piece(string);
            assert_eq!(result, Some(expected_piece));
        }
    }

    #[test]
    fn test_parse_hand_piece_incorrect() {
        let inputs = [
            "", "x", "123", "n", "g", "s", // shogi pieces not in Let's catch the Lion
        ];

        for string in inputs {
            let result = parse_hand_piece(string);
            assert_eq!(result, None);
        }
    }

    #[test]
    fn test_hand_lexer_simple() {
        let inputs = [
            ("", vec![]),
            (
                "rbpRBP",
                vec![
                    HandLexerOutput {
                        piece: HandPiece::Rook,
                        color: Color::White,
                        count: 1,
                    },
                    HandLexerOutput {
                        piece: HandPiece::Bishop,
                        color: Color::White,
                        count: 1,
                    },
                    HandLexerOutput {
                        piece: HandPiece::Pawn,
                        color: Color::White,
                        count: 1,
                    },
                    HandLexerOutput {
                        piece: HandPiece::Rook,
                        color: Color::Black,
                        count: 1,
                    },
                    HandLexerOutput {
                        piece: HandPiece::Bishop,
                        color: Color::Black,
                        count: 1,
                    },
                    HandLexerOutput {
                        piece: HandPiece::Pawn,
                        color: Color::Black,
                        count: 1,
                    },
                ],
            ),
        ];

        for (string, expected) in inputs {
            let result = lex_hand(string);
            assert_eq!(result, Some(expected));
        }
    }

    #[test]
    fn test_hand_lexer_simple_with_counts() {
        let inputs = [
            ("", vec![]),
            (
                "r2b2P2",
                vec![
                    HandLexerOutput {
                        piece: HandPiece::Rook,
                        color: Color::White,
                        count: 2,
                    },
                    HandLexerOutput {
                        piece: HandPiece::Bishop,
                        color: Color::White,
                        count: 2,
                    },
                    HandLexerOutput {
                        piece: HandPiece::Pawn,
                        color: Color::Black,
                        count: 2,
                    },
                ],
            ),
        ];

        for (string, expected) in inputs {
            let result = lex_hand(string);
            assert_eq!(result, Some(expected));
        }
    }

    #[test]
    fn test_parse_hands_empty() {
        let input = "";

        let result = parse_hands(input).unwrap();

        assert_eq!(result.get(&Color::Black).unwrap(), &Hand(HashMap::new()));
        assert_eq!(result.get(&Color::White).unwrap(), &Hand(HashMap::new()));
    }

    #[test]
    fn test_parse_hands_full() {
        let input = "pr2PB2";

        let result = parse_hands(input).unwrap();

        let black_hand = result.get(&Color::Black).unwrap();
        let white_hand = result.get(&Color::White).unwrap();

        assert_eq!(white_hand.0.get(&HandPiece::Pawn), Some(&1));
        assert_eq!(white_hand.0.get(&HandPiece::Bishop), None);
        assert_eq!(white_hand.0.get(&HandPiece::Rook), Some(&2));

        assert_eq!(black_hand.0.get(&HandPiece::Pawn), Some(&1));
        assert_eq!(black_hand.0.get(&HandPiece::Bishop), Some(&2));
        assert_eq!(black_hand.0.get(&HandPiece::Rook), None);
    }
}
