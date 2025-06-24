mod board;
mod game;
mod gfx;
use game::Game;

use std::cmp::min;
use std::fs;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
};
const ROWS: usize = 14;
const COLS: usize = 12;
const NUM: u32 = 125;
fn screen_to_grid(x: f64, y: f64, pixel_scale: u32, pixels_per_cell: u32) -> (usize, usize) {
    let cell_size = pixel_scale * pixels_per_cell;
    let grid_x = min((x as u32 / cell_size) as usize, COLS);
    let grid_y = min((y as u32 / cell_size) as usize, ROWS);
    (grid_x, grid_y)
}

fn main() {
    let file_string = format!("flows/{}x{}_{}.txt", COLS, ROWS, NUM);
    let board_string: String = fs::read_to_string(file_string).unwrap();

    let mut game = Game::new(&board_string);
    let (mut gfx, event_loop) = gfx::Gfx::new(COLS as u32, ROWS as u32);
    let (mut row, mut col) = (0, 0);
    let (mut last_row, mut last_col) = (0, 0);

    event_loop.run(move |event, _, control_flow| {
        if game.is_finished() {
            *control_flow = ControlFlow::Wait;
        } else {
            *control_flow = ControlFlow::Poll;
        }

        match event {
            Event::MainEventsCleared => {
                if !game.is_finished() {
                    if game.update() {
                        println!("You win!");
                        gfx.window.set_title("You Win!");
                    }
                }
                gfx.flow4_display(game.get_board());
                gfx.render();
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                // Mouse button press/release
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
                    let (new_col, new_row) = screen_to_grid(
                        position.x,
                        position.y,
                        gfx::PIXEL_SCALE,
                        gfx::PIXELS_PER_CELL,
                    );

                    if new_row != row || new_col != col {
                        last_row = row;
                        last_col = col;

                        row = new_row;
                        col = new_col;

                        game.handle_mouse_move(row, col);
                    }
                }

                // Mouse wheel
                // WindowEvent::MouseWheel { delta, .. } => match delta {
                //     winit::event::MouseScrollDelta::LineDelta(x, y) => {
                //         println!("Mouse wheel: ({}, {})", x, y);
                //     }
                //     winit::event::MouseScrollDelta::PixelDelta(pos) => {
                //         println!("Mouse wheel pixel delta: ({}, {})", pos.x, pos.y);
                //     }
                // },
                _ => {}
            },

            _ => {}
        }
    });
}
