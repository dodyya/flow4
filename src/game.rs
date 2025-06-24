use crate::COLS;
use crate::ROWS;
use crate::board::{
    Board, Cell, ind, is_legal, is_solved, load_board, neighbor_head, num_colors, orientation,
    set_cell, strip_board,
};

pub struct Game {
    board: Board,
    dragging: bool,
    color: usize,
    finished: bool,
    flows: Vec<Flow>,
}

#[derive(Debug, Clone)]
struct Flow {
    pub cells: Vec<(usize, usize)>,
    complete: bool,
}

impl Flow {
    fn new() -> Self {
        Flow {
            cells: Vec::new(),
            complete: false,
        }
    }

    fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    fn add_cell(&mut self, row: usize, col: usize) {
        self.cells.push((row, col));
    }

    fn reset_to(&mut self, row: usize, col: usize) {
        self.cells = vec![(row, col)];
    }

    fn set_complete(&mut self) {
        self.complete = true;
    }

    fn cut_at(&mut self, row: usize, col: usize) {
        let idx = self
            .cells
            .iter()
            .position(|(r, c)| *r == row && *c == col)
            .expect("Cell not found");
        if self.complete {
            if idx < self.cells.len() / 2 {
                self.cells = self.cells[idx..].to_vec().into_iter().rev().collect();
            } else {
                self.cells = self.cells[..idx + 1].to_vec();
            }
        } else {
            self.cells = self.cells[..idx + 1].to_vec();
        }

        self.complete = false;
    }

    fn cut_before(&mut self, row: usize, col: usize) {
        let idx = self
            .cells
            .iter()
            .position(|(r, c)| *r == row && *c == col)
            .expect("Cell not found");
        if self.complete {
            if idx < self.cells.len() / 2 {
                self.cells = self.cells[idx + 1..].to_vec().into_iter().rev().collect();
            } else {
                self.cells = self.cells[..idx].to_vec();
            }
        } else {
            self.cells = self.cells[..idx].to_vec();
        }
        self.complete = false;
    }

    fn restart(&mut self) {
        self.cells = vec![*self.cells.last().unwrap()];
        self.complete = false;
    }
}

impl Game {
    pub fn new(board_string: &str) -> Self {
        let mut board = load_board(board_string);
        strip_board(&mut board);

        Game {
            board,
            dragging: false,
            color: 0,
            finished: false,
            flows: vec![Flow::new(); num_colors(&board)],
        }
    }

    pub fn clear_flows(&mut self) {
        self.flows = vec![Flow::new(); num_colors(&self.board)];
    }

    pub fn get_board(&mut self) -> &Board {
        &self.board
    }

    // pub fn is_dragging(&self) -> bool {
    //     self.dragging
    // }

    fn update_board(&mut self) {
        strip_board(&mut self.board);
        for color in 0..self.flows.len() {
            for (row, col) in self.flows[color].cells.iter() {
                if !Cell::is_head(&self.board[ind(*row, *col)]) {
                    set_cell(
                        &mut self.board,
                        *row,
                        *col,
                        Cell::Path { color: color as u8 },
                    );
                }
            }
        }
        if !is_legal(&self.board) {
            self.flows[self.color].cells.clear();
            self.dragging = false;
            self.update_board();
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn update(&mut self) -> bool {
        if !self.finished && is_solved(&self.board) {
            self.finished = true;
            return true; // Signal that we just won
        }
        false
    }

    pub fn handle_mouse_press(&mut self, row: usize, col: usize) {
        if self.finished {
            return;
        }

        let c = self.board[ind(row, col)];
        if !c.is_empty() {
            self.color = c.color() as usize;
            if c.is_head() {
                self.flows[self.color].reset_to(row, col);
            } else {
                self.flows[self.color].cut_at(row, col);
            }
            self.update_board();
            self.dragging = true;
        }
    }

    pub fn handle_mouse_release(&mut self) {
        self.dragging = false;
    }

    pub fn handle_mouse_move(&mut self, row: usize, col: usize) {
        if self.finished || !self.dragging {
            return;
        }

        let cell = self.board[ind(row, col)];

        if cell.is_head()
            && cell.color() == self.color as u8
            && !self.flows[self.color].cells.contains(&(row, col))
        {
            self.flows[self.color].add_cell(row, col);
            self.flows[self.color].set_complete();
        } else if cell.is_empty() {
            if self.flows[self.color].complete {
                self.flows[self.color].restart();
            }
            self.flows[self.color].add_cell(row, col);
        } else if cell.color() == self.color as u8 {
            self.flows[self.color].cut_at(row, col);
        } else if !cell.is_head() {
            self.flows[self.color].add_cell(row, col);
            self.flows[cell.color() as usize].cut_before(row, col);
        } else {
            self.dragging = false;
        }

        let neighbor_result = neighbor_head(self.board, row, col, self.color as u8);
        if neighbor_result.is_some()
            && !self.flows[self.color]
                .cells
                .contains(&neighbor_result.unwrap())
        {
            self.flows[self.color].add_cell(neighbor_result.unwrap().0, neighbor_result.unwrap().1);
            self.flows[self.color].set_complete();
        }
        self.update_board();
        // println!("{}", orientation(&self.board, row, col));
    }

    pub fn handle_right_click(&mut self) {
        if self.finished {
            return;
        }
        strip_board(&mut self.board);
        self.clear_flows();
    }
}
