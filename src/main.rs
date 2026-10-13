use bracket_lib::terminal::{main_loop, BTermBuilder};
use bracket_lib::prelude::*;

// a few consts that we will use to define our game size
const GAME_HEIGHT: isize = 80;
const GAME_WIDTH: isize = 80;
const SQUARE_WIDTH: isize = GAME_WIDTH / 8;

// Trait to handle movement logic
trait Move {
    fn test_move(&mut self, target_x: i8, target_y: i8, board: Vec<Piece>) -> bool;
}

// we have a limited number of options for pieces, so we'll represent them using an enum. This can be found in Sebesta's book in section 6.4
// The usefulness of enumeration constants is found in 6.4.3, wheree we see that it helps our readability and reliability.
enum Name {
    Pawn,
    King,
    Queen,
    Rook,
    Knight,
    Bishop
}
// here's our struct to handle the gamestate. This struct is required by bracket-lib
struct State {
    // here we use a vector to store all the pieces on the board. Vectors are an implementation of an array, which can be found in section 6.5 of Sebesta's Concepts in Programming Languages book
    board: Vec<Piece>,
    // we can keep track of who's turn it is using a bool. True is white, false is black
    turn: bool,
    // each time they left click we keep track of it using this tuple. Tuples are discussed in section 6.8 of Sebesta's book
    last_left_click: (i16, i16)
}
// implement state
impl State {
    fn new(start: Vec<Piece>) -> State {
        // left click starts at negative which represents no click
        return State{ board: start, turn: true, last_left_click: (-1, -1)};
    }
    // when they left click this method activates. Returns nothing but assigns last_left_click and triggers the try_move function
    fn left_click(&mut self, bterm: &mut BTerm) {
        // this is the square they were trying to click
        let square = find_square_from_pos(bterm.mouse_pos.0, bterm.mouse_pos.1);
        // if we haven't click anywhere before, assign the click to last_left_click
        if self.last_left_click.0 < 0 {
            self.last_left_click = square;
        }
        // otherwise
        else {
            // check if it's the same spot, if so ignore
            if find_square_from_pos(bterm.mouse_pos.0, bterm.mouse_pos.1) == self.last_left_click {
                return;
            }
            // otherwise try to make a move!
            self.try_move(self.last_left_click.0 as i8, self.last_left_click.1 as i8, square.0 as i8, square.1 as i8);
        }
    }
    // is the game over yet? Returns -1 if the game is not over, returns 0 for a draw, 1 for black winning, 2 for white winning.
    fn check_for_end(&mut self) -> i8 {
        let all_moves = generate_moves(self.board.clone(), self.turn);
        for i in 0..all_moves.len() {
            if (!king_attacked(self.board.clone(), all_moves[i].0, all_moves[i].1, all_moves[i].2, self.turn)) {
                return -1;
            }
        }
        if king_attacked(self.board.clone(), all_moves[0].0, self.board[all_moves[0].0].x, self.board[all_moves[0].0].y, self.turn) {
            return if self.turn {1} else {2}
        }
        return 0;
    }
    // what piece is on this square? Returns a character representation of the piece. or a null char if there's no piece
    // this helps us with rendering pieces.
    fn piece_on_square(&mut self, x: i8, y: i8) -> char {
        for i in 0..self.board.len() {
            if self.board[i].x == x  && self.board[i].y == y {
                return self.board[i].glyph;
            }
        }
        return '\0';
    }
    // try to make a move! Returns true if successful, false if not.
    // takes in params for the start coordinates and target coordinates
    fn try_move(&mut self, initial_x: i8, initial_y: i8, target_x: i8, target_y: i8) -> bool {
        // first let's reset our left click.
        self.last_left_click = (-1, -1);
        // define a variable for who's trying to move
        let mut mover_ind: usize = 33;
        // iterate through the board looking for our mover
        for i in 0..self.board.len() {
            if self.board[i].x == initial_x && self.board[i].y == initial_y {
                mover_ind = i;
            }
        }
        // there can always be a maximum of 32 pieces on the board so if the index is more than 32 we know we didn't find any piece at the initial square
        if mover_ind > 32 {
            return false;
        }
        // if it's the wrong color, we can't move
        if self.board[mover_ind].color != self.turn {
            return false;
        }
        // if this move would result in us being in check, we can't make the move.
        if king_attacked(self.board.clone(), mover_ind, target_x, target_y, self.turn) {
            return false;
        } 
        // this var is so that Rust doesn't get mad at us for referencing self.board twice in the same line.
        let tmp_board = self.board.clone();
        // outsource the logic of the move itself to the Piece object
        // if it finds that the move is illegal by the rules, it returns false.
        if !self.board[mover_ind].test_move(target_x, target_y, tmp_board) {
            return false;
        }
        // Iterate through the array and check if we're capturing a piece
        for i in 0..self.board.len() {
            if self.board[i].x == target_x && self.board[i].y == target_y {
                // if we're trying to capture our own piece, that's illegal. No friendly fire!
                if self.board[i].color == self.board[mover_ind].color {
                    return false;
                }
                // otherwise, get rid of the weakling
                self.board.remove(i);
                if mover_ind > i {
                    mover_ind -= 1;
                }
                break;
            }
        }
        // set our new position to target position
        self.board[mover_ind].x = target_x; 
        self.board[mover_ind].y = target_y;
        // this is here so that we can track pawn's double movements.
        self.board[mover_ind].has_moved = true;
        // other guy's turn!
        self.turn = !self.turn;
        // hey it worked!
        return true;
    }
    // this function handles the rendering of the game
    // it is called each tick and takes in a mutable bterm param
    fn render(&mut self, bterm: &mut BTerm) {
        // clear the screen! make the background white
        bterm.cls_bg(WHITE);
        // is it over?
        let game_result = self.check_for_end();
        // yes!
        if game_result >= 0{
            bterm.print_color_centered(40, WHITE, BLACK, if game_result == 0 {"DRAW!"} else if game_result == 1 {"BLACK WINS!"} else {"WHITE WINS!"});
        }
        // no!
        else {
            // iterate through each column
            for i in 0..8 {
                // and each row
                for x in 0..8 {
                    // create a rect for the square
                    let square = Rect::with_size(x * SQUARE_WIDTH, 70 - i * SQUARE_WIDTH, SQUARE_WIDTH, SQUARE_WIDTH);
                    // start with a var to track color so we can make the text opposite to the square
                    let mut square_color = WHITE;
                    // if they are both even or both odd the square is black, color it so.
                    if i % 2 == x % 2 {
                        square_color = BLACK;
                        bterm.fill_region(
                            square,
                            '\0',
                            WHITE,
                            BLACK
                        );
                    }
                    // print the text for the piece!
                    bterm.print_color_centered_at(
                        x * SQUARE_WIDTH + (SQUARE_WIDTH / 2),
                        (7 - i) * SQUARE_WIDTH + (SQUARE_WIDTH / 2),
                        // if there's a black piece there we want to differentiate them by coloring the text red and the background grey-ish. So we use if elses
                        if contains_piece(self.board.clone(), x as i8, i as i8) && !piece_at(self.board.clone(), x as i8, i as i8).color { 
                            RED 
                        }
                        else if square_color == WHITE {
                            BLACK
                        } else {
                            WHITE
                        },
                    if contains_piece(self.board.clone(), x as i8, i as i8) && !piece_at(self.board.clone(), x as i8, i as i8).color {
                            (64, 64, 64)
                        }
                        else if square_color == WHITE {
                            WHITE
                        }
                        else {BLACK},
                        self.piece_on_square((x as i8), (i as i8))
                    ); 
                }
            }
            // if they've clicked, use the helper function for left clicks
            if bterm.left_click {
                self.left_click(bterm);
            }
        }
    }
}

// generate every possible move
fn generate_moves(mut board: Vec<Piece>, turn: bool) -> Vec<(usize, i8, i8)> {
    let mut result: Vec<(usize, i8, i8)> = Vec::new();
    let tmp_clone = board.clone();
    for i in 0..board.len() {
        if board[i].color != turn {
            continue;
        }
        for x in 0..8 {
            for y in 0..8 {
                if board[i].test_move(x, y, tmp_clone.clone()) {
                    if contains_piece(board.clone(), x, y) {
                        if piece_at(board.clone(), x, y).color == board[i].color { continue; }
                    }
                    result.push((i, x, y));
                }
            }
        }
    }
    return result;
}

fn king_attacked(board: Vec<Piece>, mover: usize, x: i8, y: i8, turn: bool) -> bool {
    let mut tmp_board = board.clone();
    tmp_board[mover].x = x;
    tmp_board[mover].y = y;
    let mut king_target_x: i8 = 64;
    let mut king_target_y: i8 = 64;
    for i in 0..tmp_board.len() {
        if tmp_board[i].color == turn && matches!(tmp_board[i].piece, Name::King) {
            king_target_x = tmp_board[i].x;
            king_target_y = tmp_board[i].y;
        }
    }
    let copy_board = tmp_board.clone();
    for i in 0..tmp_board.len() {
        if tmp_board[i].color == turn {
            continue;
        }
        if tmp_board[i].test_move(king_target_x, king_target_y, copy_board.clone()) {
            if tmp_board[i].x == x && tmp_board[i].y == y {
                continue;
            }
            return true;
        }
    }
    return false;
}

fn find_square_from_pos(pos_x: i32, pos_y: i32) -> (i16, i16){
    let x_square = pos_x / 80;
    // because the pixels count from top left down, we have to reverse our y value
    let y_square = 7 - pos_y / 80;
    return (x_square as i16, y_square as i16);
}

impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        self.render(bterm);
        if self.last_left_click.0 < 1 {
            return;
        }
        if !contains_piece(self.board.clone(), self.last_left_click.0 as i8, self.last_left_click.1 as i8) {
            self.last_left_click = (-1, -1)
        }
    }
}

struct Piece {
    piece: Name,
    x: i8,
    y: i8,
    color: bool, // white is true, black is false
    has_moved: bool,
    glyph: char
}

impl Piece {
    fn new(name: Name, xpos: i8, ypos: i8, col: bool) -> Piece {
        Piece {
            // here we see the usefulness of our enum and the match function that rust gives us
            glyph: match name {
                Name::Bishop => 'B',
                Name::King => 'K',
                Name::Rook => 'R',
                Name::Knight => 'N',
                Name::Queen => 'Q',
                Name::Pawn => 'P'
            },
            piece: name,
            x: xpos,
            y: ypos,
            color: col,
            has_moved: false
        }
    }
}

impl Move for Piece {
    fn test_move(&mut self, target_x: i8, target_y: i8, board: Vec<Piece>) -> bool {
        if self.x == target_x && self.y == target_y {return false;}
        let dir: i8 = if self.color {1} else {-1};
        match self.piece {
            // we can move forward one, forward two, or capture diagonally.
            // dir is so that black pawns move down and white pawns move up.
            Name::Pawn => (target_x == self.x && target_y == (self.y + dir) && !contains_piece(board.clone(), target_x, target_y)) 
                || (target_x == self.x && !self.has_moved && target_y == (self.y + dir * 2) && !contains_piece(board.clone(), target_x, target_y) && !contains_piece(board.clone(), target_x, self.y + dir)) 
                || ((target_x == self.x - 1 || target_x == self.x + 1) && target_y == (self.y + dir)&& contains_piece(board.clone(), target_x, target_y)),
            // here we can move our x value 2 and our y value 1, or vice versa
            Name::Knight => ((target_x - self.x).abs() == 2 && (target_y - self.y).abs() == 1) 
                || (target_x - self.x).abs() == 1 && (target_y - self.y).abs() == 2,
            // king can move no more than one square in any direction
            Name::King => (target_x - self.x).abs() <= 1 && (target_y - self.y).abs() <= 1,
            // bishop can move diagonally as far as it wants, so long as there's nothing blocking it
            Name::Bishop => (target_x - self.x).abs() == (target_y - self.y).abs() && !piece_between(board.clone(), self.x, self.y, target_x, target_y),
            // just rook and bishop combined with an or
            Name::Queen => (target_x - self.x).abs() == (target_y - self.y).abs() && !piece_between(board.clone(), self.x, self.y, target_x, target_y) 
                || ((target_x == self.x) ^ (target_y == self.y)) && !piece_between(board.clone(), self.x, self.y, target_x, target_y),
            // The rook can move as far as it wants, so long as either x or y stay the same (it can't move diagonally)
            Name::Rook => ((target_x == self.x) ^ (target_y == self.y)) && !piece_between(board.clone(), self.x, self.y, target_x, target_y)
        }
    }
}
// these help us with the messiness of Rust's mutability caution since we can clone our board vector now
impl Clone for Piece {
    fn clone(&self) -> Piece {
        return Piece::new(self.piece.clone(), self.x, self.y, self.color);
    }
}
// this also is necessary to clone a Piece
impl Clone for Name {
    fn clone(&self) -> Name {
        match *self {
            Name::Pawn => Name::Pawn,
            Name::King => Name::King,
            Name::Queen => Name::Queen,
            Name::Rook => Name::Rook,
            Name::Knight => Name::Knight,
            Name::Bishop => Name::Bishop,
        }
    }
}
fn main() {
    // create our board
    let mut board: Vec<Piece> = Vec::new();
    // starting pos
    instantiate_board(&mut board);
    let mut bterm = BTermBuilder::simple(GAME_HEIGHT, GAME_WIDTH).expect("Render failed").with_title("Chess").build().expect("Build failed");
    //bterm.set_active_font(8, false);
    let _ = main_loop(bterm, State::new(board));

}

// helper function to get the starting position set up.
fn instantiate_board(board: &mut Vec<Piece>) {
    // white pieces.
    board.push(Piece::new(Name::Rook, 0, 0, true));
    board.push(Piece::new(Name::Knight, 1, 0, true));
    board.push(Piece::new(Name::Bishop, 2, 0, true));
    board.push(Piece::new(Name::Queen, 3, 0, true));
    board.push(Piece::new(Name::King, 4, 0, true));
    board.push(Piece::new(Name::Bishop, 5, 0, true));
    board.push(Piece::new(Name::Knight, 6, 0, true));
    board.push(Piece::new(Name::Rook, 7, 0, true));
    for i in 0..8 { // loop for white pawns
        board.push(Piece::new(Name::Pawn,i, 1, true));
    }
    for i in 0..8 { //loop for black pawns
        board.push(Piece::new(Name::Pawn, i, 6, false));
    }
    // black pieces
    board.push(Piece::new(Name::Rook,0, 7, false));
    board.push(Piece::new(Name::Knight,1, 7, false));
    board.push(Piece::new(Name::Bishop,2, 7, false));
    board.push(Piece::new(Name::Queen,3, 7, false));
    board.push(Piece::new(Name::King,4, 7, false));
    board.push(Piece::new(Name::Bishop,5, 7, false));
    board.push(Piece::new(Name::Knight, 6, 7, false));
    board.push(Piece::new(Name::Rook,7, 7, false));
}


// check if a square contains a piece
fn contains_piece(board: Vec<Piece>, x: i8, y: i8) -> bool {
    for i in 0..board.len() {
        if board[i].x == x && board[i].y == y {
            return true;
        }
    }
    return false;
}
// check if there is a piece blocking a path.
// utilizes loops to check each square on the path
fn piece_between(board: Vec<Piece>, start_x: i8, start_y: i8, end_x: i8, end_y: i8) -> bool {
    if start_x == end_x {
        for y in (start_y + 1)..end_y {
            if contains_piece(board.clone(), start_x, y) {
                return true;
            }
        }
        return false;
    }
    else if start_y == end_y {
        for x in (start_x + 1)..end_x {
            if contains_piece(board.clone(), x, start_y) {
                return true;
            }
        }
        return false;
    }
    else {
        if (start_x - end_x).abs() != (start_y - end_y).abs() {
            return false;
        }
        let dir_x = if start_x < end_x {1} else {-1};
        let dir_y = if start_y < end_y {1} else {-1};
        for i in 1..(start_x - end_x).abs() {
            if contains_piece(board.clone(), start_x + i * dir_x, start_y + i * dir_y) {
                return true;
            }
        }
        return false;
    }
}

// return a copy of the piece at a square
fn piece_at(board: Vec<Piece>, x: i8, y: i8) -> Piece {
    for i in 0..board.len() {
        if board[i].x == x && board[i].y == y {
            return board[i].clone();
        }
    }
    return board[0].clone();
}
