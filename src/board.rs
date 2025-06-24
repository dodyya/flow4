use crate::game::Game;

const ROWS: usize = Game::ROWS;
const COLS: usize = Game::COLS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Path { color: u8 },
    Head { color: u8 },
}

impl Cell {
    pub fn color(&self) -> u8 {
        match self {
            Cell::Empty => panic!(),
            Cell::Path { color } => *color,
            Cell::Head { color } => *color,
        }
    }

    pub fn is_head(&self) -> bool {
        match self {
            Cell::Head { color } => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

// AdDcC
// adEcB
// adecB
// aDeCb
// aAEBb

pub type Board = [Cell; ROWS * COLS];

pub fn new() -> Board {
    [Cell::Empty; ROWS * COLS]
}

pub fn num_colors(board: &Board) -> usize {
    //Basically means that colors should be alphabetic
    let mut max_color = 0;
    for cell in board {
        if let Cell::Head { color } = cell {
            if *color > max_color {
                max_color = *color;
            }
        }
    }
    max_color as usize + 1
}

///Takes a row, col index and returns the index in the flat array
pub fn ind(row: usize, col: usize) -> usize {
    if row >= ROWS || col >= COLS {
        panic!();
    }
    row * COLS + col
}

pub fn inverse_ind(index: usize) -> (usize, usize) {
    (index / COLS, index % COLS)
}

///Returns a cell's 4 neighbors, padding out of bounds neighbors with Empty
fn get_neighbors(board: Board, row: usize, col: usize) -> [Cell; 4] {
    let mut out = [Cell::Empty; 4];
    if row > 0 {
        out[0] = board[ind(row - 1, col)];
    }

    if col > 0 {
        out[1] = board[ind(row, col - 1)];
    }

    if row < ROWS - 1 {
        out[2] = board[ind(row + 1, col)];
    }

    if col < COLS - 1 {
        out[3] = board[ind(row, col + 1)];
    }
    out
}

pub fn neighbor_head(board: Board, row: usize, col: usize, color: u8) -> Option<(usize, usize)> {
    let target = Cell::Head { color };
    if row > 0 {
        if board[ind(row - 1, col)] == target {
            return Some((row - 1, col));
        }
    }
    if row < ROWS - 1 {
        if board[ind(row + 1, col)] == target {
            return Some((row + 1, col));
        }
    }
    if col > 0 {
        if board[ind(row, col - 1)] == target {
            return Some((row, col - 1));
        }
    }
    if col < COLS - 1 {
        if board[ind(row, col + 1)] == target {
            return Some((row, col + 1));
        }
    }
    return None;
}

//For graphics: Every single path cell can be drawn by describing the neighbors of the same color it has in the following way:
//

pub fn orientation(&board: &Board, index: usize) -> u8 {
    let mut out = 0;
    let (row, col) = inverse_ind(index);
    let neighbors = get_neighbors(board, row, col);
    let own_color = board[index].color();
    for i in 0..4 {
        if !neighbors[i].is_empty() && neighbors[i].color() == own_color {
            out += 1 << i;
        }
    }
    out
}

pub fn is_solved(&board: &Board) -> bool {
    for i in 0..ROWS {
        for j in 0..COLS {
            match board[ind(i, j)] {
                Cell::Empty => return false,
                Cell::Path { color } => {
                    if num_neighbors_of_color(board, color, i, j) != 2 {
                        return false;
                    }
                }
                Cell::Head { color } => {
                    if num_neighbors_of_color(board, color, i, j) != 1 {
                        return false;
                    }
                }
            };
        }
    }
    return true;
}

///Checks whether paths are laid out legally
pub fn is_legal(&board: &Board) -> bool {
    for i in 0..ROWS {
        for j in 0..COLS {
            match board[ind(i, j)] {
                Cell::Empty => {}
                Cell::Path { color } => {
                    let n = num_neighbors_of_color(board, color, i, j);
                    if n > 2 || n == 0 {
                        return false;
                    }
                }
                Cell::Head { color } => {
                    if num_neighbors_of_color(board, color, i, j) > 1 {
                        return false;
                    }
                }
            };
        }
    }
    return true;
}

///Checks whether all colors appear only twice
pub fn is_valid(&board: &Board) -> bool {
    let mut seen_colors: u64 = 0; //Double the number of possible colors
    for cell in board {
        match cell {
            Cell::Head { color } => {
                seen_colors += 1 << (color * 2);
                if (seen_colors & 3 << (color * 2)) == 3 << color {
                    return false;
                }
            }
            _ => {}
        }
    }
    seen_colors & 0x5555_5555_5555_5555 == 0
}

fn num_neighbors_of_color(board: Board, color: u8, row: usize, col: usize) -> u32 {
    get_neighbors(board, row, col).iter().fold(0, |sum, c| {
        if c != &Cell::Empty && color == c.color() {
            sum + 1
        } else {
            sum
        }
    })
}

// fn is_impossible(b: Board) -> bool {}

pub fn load_board(board: &str) -> Board {
    let board_vec = board
        .bytes()
        .filter(|byte| byte.is_ascii_alphanumeric() || byte.is_ascii_punctuation())
        .map(|byte| match byte {
            b'a'..=b'z' => Cell::Path {
                color: (byte - b'a') as u8,
            },
            b'A'..=b'Z' => Cell::Head {
                color: (byte - b'A') as u8,
            },
            _ => Cell::Empty,
        })
        .collect::<Vec<Cell>>();

    board_vec.try_into().unwrap()
}

pub fn set_cell(board: &mut Board, row: usize, col: usize, cell: Cell) {
    board[ind(row, col)] = cell;
}

pub fn strip_board(board: &mut Board) {
    board.iter_mut().for_each(|cell| match cell {
        Cell::Head { color } => *cell = Cell::Head { color: *color },
        _ => *cell = Cell::Empty,
    });
}

pub fn print_board(board: &Board) {
    for i in 0..ROWS {
        for j in 0..COLS {
            let c = match board[ind(i, j)] {
                Cell::Empty => '.',
                Cell::Path { color } => (color + b'a') as char,
                Cell::Head { color } => (color + b'A') as char,
            };
            print!("{}", c);
        }
        println!();
    }
}
