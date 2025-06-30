use std::ops::{Index, IndexMut};

use crate::COLS;
use crate::ROWS;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    rows: usize,
    cols: usize,
    cells: Box<[Cell]>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        Self {
            rows,
            cols,
            cells: vec![Cell::Empty; rows * cols].into_boxed_slice(),
        }
    }

    pub fn num_colors(&self) -> usize {
        let mut colors = Vec::new();
        for i in self.cells.iter().filter(|cell| cell.is_head()) {
            if !colors.contains(&i.color()) {
                colors.push(i.color());
            }
        }
        colors.len()
    }

    ///Returns a cell's 4 neighbors, padding out of bounds neighbors with Empty
    fn neighbors_or_empty(&self, ind: (usize, usize)) -> [Cell; 4] {
        let (row, col) = ind;
        let mut out = [Cell::Empty; 4];
        if row > 0 {
            out[0] = self[(row - 1, col)];
        }

        if col > 0 {
            out[1] = self[(row, col - 1)];
        }

        if row < ROWS - 1 {
            out[2] = self[(row + 1, col)];
        }

        if col < COLS - 1 {
            out[3] = self[(row, col + 1)];
        }
        out
    }

    ///Returns a cell's 4 empty neighbors. Edges or nonempty are represented as None.
    pub fn empty_neighbors(&self, row: usize, col: usize) -> [Option<(usize, usize)>; 4] {
        let mut out = [None; 4];
        if row > 0 {
            if self[(row - 1, col)] == Cell::Empty {
                out[0] = Some((row - 1, col));
            }
        }

        if col > 0 {
            if self[(row, col - 1)] == Cell::Empty {
                out[1] = Some((row, col - 1));
            }
        }

        if row < ROWS - 1 {
            if self[(row + 1, col)] == Cell::Empty {
                out[2] = Some((row + 1, col));
            }
        }

        if col < COLS - 1 {
            if self[(row, col + 1)] == Cell::Empty {
                out[3] = Some((row, col + 1));
            }
        }
        out
    }

    pub fn neighbor_head(&self, row: usize, col: usize, color: u8) -> Option<(usize, usize)> {
        let target = Cell::Head { color };
        if row > 0 {
            if self[(row - 1, col)] == target {
                return Some((row - 1, col));
            }
        }
        if row < ROWS - 1 {
            if self[(row + 1, col)] == target {
                return Some((row + 1, col));
            }
        }
        if col > 0 {
            if self[(row, col - 1)] == target {
                return Some((row, col - 1));
            }
        }
        if col < COLS - 1 {
            if self[(row, col + 1)] == target {
                return Some((row, col + 1));
            }
        }
        return None;
    }

    ///For graphics: Every single path cell can be drawn by describing the neighbors of the same color it has in the following way:
    pub fn orientation(&self, index: usize) -> u8 {
        let mut out = 0;
        let neighbors = self.neighbors_or_empty(Self::inverse_ind(index));
        let own_color = self[index].color();
        for i in 0..4 {
            if !neighbors[i].is_empty() && neighbors[i].color() == own_color {
                out |= 1 << i;
            }
        }
        out
    }

    pub fn inverse_ind(index: usize) -> (usize, usize) {
        (index / COLS, index % COLS)
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..ROWS {
            for j in 0..COLS {
                match self[(i, j)] {
                    Cell::Empty => return false,
                    Cell::Path { color } => {
                        if self.num_neighbors_of_color(color, i, j) != 2 {
                            return false;
                        }
                    }
                    Cell::Head { color } => {
                        if self.num_neighbors_of_color(color, i, j) != 1 {
                            return false;
                        }
                    }
                };
            }
        }
        return true;
    }

    ///Checks whether paths are laid out legally
    pub fn is_legal(&self) -> bool {
        for i in 0..ROWS {
            for j in 0..COLS {
                match self[(i, j)] {
                    Cell::Empty => {}
                    Cell::Path { color } => {
                        let n = self.num_neighbors_of_color(color, i, j);
                        if n > 2 || n == 0 {
                            return false;
                        }
                    }
                    Cell::Head { color } => {
                        if self.num_neighbors_of_color(color, i, j) > 1 {
                            return false;
                        }
                    }
                };
            }
        }
        return true;
    }

    ///Checks whether all colors appear only twice
    pub fn is_valid(&self) -> bool {
        let mut seen_colors: u64 = 0; //Double the number of possible colors
        for cell in &self.cells {
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

    fn num_neighbors_of_color(&self, color: u8, row: usize, col: usize) -> u32 {
        self.neighbors_or_empty((row, col))
            .iter()
            .fold(0, |sum, c| {
                if c != &Cell::Empty && color == c.color() {
                    sum + 1
                } else {
                    sum
                }
            })
    }

    pub fn load_board(board_str: &str, rows: usize, cols: usize) -> Self {
        let board_vec = board_str
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

        Self {
            rows,
            cols,
            cells: board_vec.into_boxed_slice(),
        }
    }

    pub fn set_cell(board: &mut Board, row: usize, col: usize, cell: Cell) {
        board[(row, col)] = cell;
    }

    pub fn add_path(&mut self, row: usize, col: usize, color: u8) {
        self[(row, col)] = Cell::Path { color };
    }

    pub fn strip(&mut self) {
        self.cells.iter_mut().for_each(|cell| match cell {
            Cell::Head { color } => *cell = Cell::Head { color: *color },
            _ => *cell = Cell::Empty,
        });
    }

    pub fn len(&self) -> usize {
        self.rows * self.cols
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let c = match self[(i, j)] {
                    Cell::Empty => '.',
                    Cell::Path { color } => (color + b'a') as char,
                    Cell::Head { color } => (color + b'A') as char,
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        if row >= self.rows || col >= self.cols {
            panic!();
        }

        &self.cells[row * COLS + col]
    }
}

impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.cells.len() {
            panic!();
        }

        &self.cells[index]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        if row >= self.rows || col >= self.cols {
            panic!();
        }

        &mut self.cells[row * COLS + col]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.cells.len() {
            panic!();
        }

        &mut self.cells[index]
    }
}

// fn is_impossible(b: Board) -> bool {}
