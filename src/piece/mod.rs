trait Move {
    fn test_move(&mut self, target: u8) -> bool;
}

pub enum Name {
    Pawn,
    King,
    Queen,
    Rook,
    Knight,
    Bishop
}



pub struct Piece {
    piece: Name,
    position: u8,
    color: bool, // white is true, black is false
    has_moved: bool
}

impl Piece {
    fn new(name: Name, pos: u8, col: bool) -> Piece {
        Piece {
            piece: name,
            position: pos,
            color: col,
            has_moved: false
        }
    }
    fn dist_to_edge(&self, dir: i8) -> u8 {
        return 0
    }
}
