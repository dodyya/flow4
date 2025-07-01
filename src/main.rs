#![allow(unused)]
#![allow(non_snake_case)]
mod board;
mod game;
mod gfx;
mod solver;
mod solver_stack;

use crate::board::Board;
use crate::solver_stack::SolverStack;
use std::thread;
use std::time::Duration;

use std::fs;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

const ROWS: usize = 8;
const COLS: usize = 8;
const NUM_PUZZLES: u32 = 150;

const SOLVING: bool = true;

fn initialize(n: u32) -> solver_stack::SolverStack {
    let file_string = format!("flows/{}x{}_{}.txt", COLS, ROWS, n);
    let board_string: String = fs::read_to_string(file_string).unwrap();
    let mut board = Board::load_board(&board_string, ROWS, COLS);
    board.strip();
    solver_stack::SolverStack::new(solver::Solver::new(&board))
}

fn main() {
    let mut n = 1;

    let mut solved = 0;
    let mut solver: SolverStack = initialize(n);
    let (mut gfx, event_loop) = gfx::Gfx::new(COLS as u32, ROWS as u32);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                // gfx.display(&Board::load_board(
                //     "dddDBb
                // dB..Cb
                // d.A..b
                // dbbc.b
                // dAbCbb
                // dDbbb.",
                //     ROWS,
                //     COLS,
                // ));
                solver.step();
                gfx.display(&solver.get_board());
                // print!("{}{}", 27 as char, 13 as char);
                // println!("{:?}", solver.failed());
                // solver.get_board().print();
                gfx.render();

                if solver.done() || solver.failed() {
                    // println!("Moving on!");
                    if !solver.failed() {
                        gfx.success_display(&solver.get_board());
                        gfx.render();
                        thread::sleep(Duration::from_millis(100));
                        solved += 1;
                    } else {
                        // *control_flow = ControlFlow::Wait;
                        gfx.fail_display(&solver.get_board());
                        gfx.render();
                        thread::sleep(Duration::from_millis(500));

                        // return;
                    }
                    n += 1;
                    if n > NUM_PUZZLES {
                        *control_flow = ControlFlow::Wait;
                        // println!("All puzzles solved");
                        return;
                    }
                    solver = initialize(n);
                    // println!("Initialized {}", n);
                    gfx.window.set_title(&format!(
                        "{}x{} level {}          Accuracy:{}/{}",
                        COLS,
                        ROWS,
                        n,
                        solved,
                        n - 1
                    ));
                }
                // thread::sleep(Duration::from_millis(500));
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                _ => {}
            },

            _ => {}
        }
    });
}

// fn window_to_grid(x: f64, y: f64, pixel_scale: u32, pixels_per_cell: u32) -> (usize, usize) {
//     let cell_size = pixel_scale * pixels_per_cell;
//     let grid_x = min((x as u32 / cell_size) as usize, COLS);
//     let grid_y = min((y as u32 / cell_size) as usize, ROWS);
//     (grid_x, grid_y)
// }
// Mouse button press/release
//     WindowEvent::MouseInput { state, button, .. } => match (state, button) {
//         (ElementState::Pressed, MouseButton::Left) => {
//             game.handle_mouse_press(row, col);
//         }
//         (ElementState::Released, MouseButton::Left) => {
//             game.handle_mouse_release();
//         }
//         (ElementState::Pressed, MouseButton::Right) => {
//             game.handle_right_click();
//         }
//         _ => {}
//     },

//     // Mouse movement
//     WindowEvent::CursorMoved { position, .. } => {
//         if position.x as f64 / gfx::PIXEL_SCALE as f64 > gfx.width as f64
//             || position.y as f64 / gfx::PIXEL_SCALE as f64 > gfx.height as f64
//         {
//             println!(
//                 "Position: {:?} Limits: {:?}",
//                 position,
//                 (gfx.width, gfx.height)
//             );
//             return;
//         }
//         let (new_col, new_row) = window_to_grid(
//             position.x,
//             position.y,
//             gfx::PIXEL_SCALE,
//             gfx::PIXELS_PER_CELL,
//         );

//         if new_row != row || new_col != col {
//             row = new_row;
//             col = new_col;

//             game.handle_mouse_move(row, col);
//         }
//     }
