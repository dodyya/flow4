mod board;
mod game;
mod gfx;
use game::Game;

use std::cmp::min;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
};

use std::fs;

fn screen_to_grid(x: f64, y: f64, pixel_scale: u32, pixels_per_cell: u32) -> (usize, usize) {
    let cell_size = pixel_scale * pixels_per_cell;
    let grid_x = min((x as u32 / cell_size) as usize, Game::COLS - 1);
    let grid_y = min((y as u32 / cell_size) as usize, Game::ROWS - 1);
    (grid_x, grid_y)
}

fn main() {
    let board_string: String = fs::read_to_string("flows/15x15_6.txt").unwrap();

    let mut game = Game::new(&board_string);
    let (mut gfx, event_loop) = gfx::Gfx::new(Game::COLS as u32, Game::ROWS as u32);
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
