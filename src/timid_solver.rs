use crate::board::{Board, Cell};

// Timid solver only has one game state, and immediately fails when encountering any
// decision (risk) it has to take.

#[derive(Copy, Clone, Debug)]
pub struct Move {
    position: (usize, usize),
    color: u8,
}

#[derive(Copy, Clone, Debug)]
struct Step {
    position: (usize, usize),
    color: u8,
}

fn adjacent(a: (usize, usize), b: (usize, usize)) -> bool {
    let (x1, y1) = (a.0 as isize, a.1 as isize);
    let (x2, y2) = (b.0 as isize, b.1 as isize);
    if x1 == x2 && (y1 == y2 + 1 || y1 == y2 - 1) {
        return true;
    }
    if y1 == y2 && (x1 == x2 + 1 || x1 == x2 - 1) {
        return true;
    }
    false
}

#[derive(Clone, Debug)]
struct Flow {
    steps: Vec<Step>,
    complete: bool,
}

impl Flow {
    fn last(&self) -> Option<&Step> {
        self.steps.last()
    }
}
pub struct Solver {
    pub flows: Vec<Flow>,
    pub board: Board,
}

impl Solver {
    pub fn new(b: &Board) -> Solver {
        let mut flows: Vec<Flow> = vec![];
        for i in 0..b.len() {
            if b[i].is_head() {
                flows.push(Flow {
                    steps: vec![Step {
                        position: Board::inverse_ind(i),
                        color: b[i].color(),
                    }],
                    complete: false,
                });
            }
        }
        Self {
            flows,
            board: b.clone(),
        }
    }

    pub fn forced_move(&self) -> Option<(Move, &Flow)> {
        for f in &self.flows {
            let moves: Vec<Move> = self.moves_from(&f.last().unwrap());
            // println!("{:?}", f);
            if moves.len() == 1 && !f.complete {
                return Some((moves[0], &f));
            }
        }
        return None;
    }

    pub fn moves_from(&self, c: &Step) -> Vec<Move> {
        let slots = self.board.empty_neighbors(c.position.0, c.position.1);
        slots
            .iter()
            .filter(|result| result.is_some())
            .map(|some| Move {
                position: some.unwrap(),
                color: c.color,
            })
            .collect()
    }

    pub fn solve_board(mut self) -> Board {
        while !self.board.is_solved() {
            self.solution_step();
        }
        self.board
    }

    pub fn solution_step(&mut self) {
        if self.board.is_solved() {
            return;
        }
        let move_result: Option<(Move, &Flow)> = self.forced_move();

        if move_result.is_some() && !move_result.as_ref().unwrap().1.complete {
            self.make(move_result.unwrap().0);
            self.check_solved_flows();
        } else {
            return;
        }
    }

    pub fn make(&mut self, m: Move) {
        self.board.add_path(m.position.0, m.position.1, m.color);

        self.add_to_appropriate_flow(m);
    }

    fn add_to_appropriate_flow(&mut self, m: Move) {
        let flow = self.flows.iter_mut().find(|flow| {
            flow.last().unwrap().color == m.color
                && adjacent(flow.last().unwrap().position, m.position)
        });
        if let Some(flow) = flow {
            flow.steps.push(Step {
                position: m.position,
                color: m.color,
            });
        }
    }

    fn check_solved_flows(&mut self) {
        //A flow is solved if: it ends near a new head of its color
        //OR one of the flows of a color ends with a position adjacent to the end of the other

        for i in 0..self.flows.len() {
            for j in 0..self.flows.len() {
                if i != j
                    && adjacent(
                        self.flows[i].last().unwrap().position,
                        self.flows[j].last().unwrap().position,
                    )
                    && self.flows[i].complete == false
                    && self.flows[j].complete == false
                    && self.flows[i].last().unwrap().color == self.flows[j].last().unwrap().color
                {
                    self.flows[i].complete = true;
                    self.flows[j].complete = true;
                }
            }
        }
    }

    pub fn done(&self) -> bool {
        self.flows.iter().all(|flow| flow.complete)
    }

    pub fn failed(&self) -> bool {
        !self.done() && self.forced_move().is_none()
    }

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }
}
