#![allow(unused)]
#![allow(non_snake_case)]
mod board;
mod game;
mod gfx;
mod solver;
mod solver_stack;

use crate::board::Board;
use crate::game::Game;
use crate::solver_stack::SolverStack;
use std::time::Duration;
use std::{thread, time::Instant};

use std::fs;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
};

use std::cmp::min;

const ROWS: usize = 15;
const COLS: usize = 15;
const NUM_PUZZLES: u32 = 150;

const SOLVING: bool = true;

fn initialize(n: u32) -> Game {
    let file_string = format!("flows/{}x{}_{}.txt", COLS, ROWS, n);
    let board_string: String = fs::read_to_string(file_string).unwrap();
    // let mut board = Board::load_board(&board_string, ROWS, COLS);
    // board.strip();
    // solver_stack::SolverStack::new(solver::Solver::new(&board))
    Game::new(&board_string)
}

fn main() {
    let mut n = 1;
    let mut game = initialize(n);
    let mut col = 0;
    let mut row = 0;
    let (mut gfx, event_loop) = gfx::Gfx::new(ROWS as u32, COLS as u32);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::MainEventsCleared = event {
            // let emu = chip8.lock().unwrap();
            gfx.display(game.get_board());
            gfx.render();

            if game.is_finished() {
                println!("Level {} complete!", n);
                n += 1;
                *control_flow = ControlFlow::WaitUntil(
                    Instant::now().checked_add(Duration::from_secs(3)).unwrap(),
                );
                game = initialize(n);
            }
        }

        match &event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::MouseInput { state, button, .. } => match (state, button) {
                        (ElementState::Pressed, MouseButton::Left) => {
                            game.handle_mouse_press(row, col);
                        }
                        (ElementState::Released, MouseButton::Left) => {
                            game.handle_mouse_release();
                        }
                        (ElementState::Pressed, MouseButton::Right) => {
                            game.handle_right_click();
                        }
                        _ => {}
                    },

                    // Mouse movement
                    WindowEvent::CursorMoved { position, .. } => {
                        if position.x as f64 / gfx::PIXEL_SCALE as f64 > gfx.width as f64
                            || position.y as f64 / gfx::PIXEL_SCALE as f64 > gfx.height as f64
                        {
                            // println!(
                            //     "Position: {:?} Limits: {:?}",
                            //     position,
                            //     (gfx.width, gfx.height)
                            // );
                            return;
                        }
                        let (new_col, new_row) = window_to_grid(
                            position.x,
                            position.y,
                            gfx::PIXEL_SCALE,
                            gfx::PIXELS_PER_CELL,
                        );

                        if new_row != row || new_col != col {
                            row = new_row;
                            col = new_col;

                            game.handle_mouse_move(row, col);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        //Mouse button press/release
    });

    // let mut n = 1;

    // let mut solved = 0;
    // let mut solver: SolverStack = initialize(n);

    // loop {
    //     if solver.done() || solver.failed() {
    //         solver.get_board().print();
    //         // println!("Moving on!");
    //         if !solver.failed() {
    //             // thread::sleep(Duration::from_millis(100));
    //             solved += 1;
    //         } else {
    //             // *control_flow = ControlFlow::Wait;
    //         }
    //         n += 1;
    //         if n > NUM_PUZZLES {
    //             break;
    //         }
    //         solver = initialize(n);
    //     }

    //     solver.step();
    // }

    // println!("Solved {} puzzles", solved);
}

fn window_to_grid(x: f64, y: f64, pixel_scale: u32, pixels_per_cell: u32) -> (usize, usize) {
    let cell_size = pixel_scale * pixels_per_cell;
    let grid_x = min((x as u32 / cell_size) as usize, COLS);
    let grid_y = min((y as u32 / cell_size) as usize, ROWS);
    (grid_x, grid_y)
}
// Mouse button press/release
// WindowEvent::MouseInput { state, button, .. } => match (state, button) {
//     (ElementState::Pressed, MouseButton::Left) => {
//         game.handle_mouse_press(row, col);
//     }
//     (ElementState::Released, MouseButton::Left) => {
//         game.handle_mouse_release();
//     }
//     (ElementState::Pressed, MouseButton::Right) => {
//         game.handle_right_click();
//     }
//     _ => {}
// },

// // Mouse movement
// WindowEvent::CursorMoved { position, .. } => {
//     if position.x as f64 / gfx::PIXEL_SCALE as f64 > gfx.width as f64
//         || position.y as f64 / gfx::PIXEL_SCALE as f64 > gfx.height as f64
//     {
//         println!(
//             "Position: {:?} Limits: {:?}",
//             position,
//             (gfx.width, gfx.height)
//         );
//         return;
//     }
//     let (new_col, new_row) = window_to_grid(
//         position.x,
//         position.y,
//         gfx::PIXEL_SCALE,
//         gfx::PIXELS_PER_CELL,
//     );

//     if new_row != row || new_col != col {
//         row = new_row;
//         col = new_col;

//         game.handle_mouse_move(row, col);
//     }
// }
