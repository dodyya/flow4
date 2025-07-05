// The timid solver fails whenever it has to make a decision. The goal of this solver is to be able to
// make a single move, and then continue onwards. If the continuation leads to a contradiction, then
// explore the other possibilities for that move. Similarly, if the single risky move leads to another risky move,
// we need to be able to continue exploring.
//
// Constraint: If we choose, at any point, to not make some move, then we can by definition never make that move again.
// E.g. when we choose to make a red end go up instead of left, we can never place a red cell where we chose to not place it,
// because it would then be adjacent. Could take advantage of this.
//

use crate::solver::Solver;

pub struct SolverStack {
    current: Solver,
    backlog: Vec<Solver>,
    failed: bool,
}

impl SolverStack {
    pub fn new(solver: Solver) -> Self {
        SolverStack {
            current: solver,
            backlog: Vec::new(),
            failed: false,
        }
    }

    pub fn step(&mut self) {
        if self.current.failed() {
            self.current = self.backlog.pop().unwrap();
            self.failed = false;
        }
        if self.current.timid_step() {
            return;
        }
        if let Some((branch1, branch2)) = self.current.binary_step() {
            self.current = branch1;
            self.backlog.push(branch2);
            self.failed = false;
        } else {
            self.failed = true;
        }
    }

    pub(crate) fn get_board(&self) -> &crate::board::Board {
        self.current.get_board()
    }

    pub(crate) fn done(&self) -> bool {
        self.current.done()
    }

    pub(crate) fn failed(&self) -> bool {
        self.failed || (self.current.failed() && self.backlog.is_empty())
    }
}
