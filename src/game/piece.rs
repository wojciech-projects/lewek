#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Bishop,
    Rook,
    King,
    PromotedPawn,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HandPiece {
    Pawn,
    Bishop,
    Rook,
}

pub const ALL_HAND_PIECES: [HandPiece; 3] = [HandPiece::Pawn, HandPiece::Bishop, HandPiece::Rook];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

pub const WHITE_PAWN: Piece = Piece {
    kind: PieceKind::Pawn,
    color: Color::White,
};
pub const WHITE_KING: Piece = Piece {
    kind: PieceKind::King,
    color: Color::White,
};
pub const WHITE_BISHOP: Piece = Piece {
    kind: PieceKind::Bishop,
    color: Color::White,
};
pub const WHITE_ROOK: Piece = Piece {
    kind: PieceKind::Rook,
    color: Color::White,
};
pub const WHITE_PROMOTED_PAWN: Piece = Piece {
    kind: PieceKind::PromotedPawn,
    color: Color::White,
};

pub const BLACK_PAWN: Piece = Piece {
    kind: PieceKind::Pawn,
    color: Color::Black,
};
pub const BLACK_KING: Piece = Piece {
    kind: PieceKind::King,
    color: Color::Black,
};
pub const BLACK_BISHOP: Piece = Piece {
    kind: PieceKind::Bishop,
    color: Color::Black,
};
pub const BLACK_ROOK: Piece = Piece {
    kind: PieceKind::Rook,
    color: Color::Black,
};
pub const BLACK_PROMOTED_PAWN: Piece = Piece {
    kind: PieceKind::PromotedPawn,
    color: Color::Black,
};

pub const ALL_PIECES: [Piece; 10] = [
    BLACK_PAWN,
    BLACK_BISHOP,
    BLACK_ROOK,
    BLACK_KING,
    BLACK_PROMOTED_PAWN,
    WHITE_PAWN,
    WHITE_BISHOP,
    WHITE_ROOK,
    WHITE_KING,
    WHITE_PROMOTED_PAWN,
];
