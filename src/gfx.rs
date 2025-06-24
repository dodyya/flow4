use hex::decode_to_slice;
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const COLORS: [&str; 24] = [
    "da0084", "f200ff", "9a9a9a", "000090", "994da4", "a85523", "f782c0", "004100", "00ffff",
    "5f0024", "4e4d50", "b59aff", "6700f9", "4c0070", "72ff9b", "6a6c00", "e2c88c", "63c2fe",
    "5a3200", "40bcab", "b49b00", "930000", "004e75", "4cb148",
];

use crate::board::Cell;
use crate::board::orientation;
use crate::board::{Board, num_colors};
use hsv::{self, hsv_to_rgb};

pub const PIXEL_SCALE: u32 = 42;
pub const PIXELS_PER_CELL: u32 = 3;
const PPC: usize = 3;

pub struct Gfx {
    pub window: Window,
    pixels: Pixels,
    pub width: u32,
    pub height: u32,
}

impl Gfx {
    pub fn new(cols: u32, rows: u32) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();
        // physical window size = virtual size × scale
        let physical_size = PhysicalSize::new(
            cols * PIXEL_SCALE * PIXELS_PER_CELL,
            rows * PIXEL_SCALE * PIXELS_PER_CELL,
        );

        let window = WindowBuilder::new()
            .with_title("Flow Four")
            .with_inner_size(physical_size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        // SurfaceTexture uses the physical (window) pixels,
        // but the 'logical' pixel buffer stays at width×height
        let surface_texture =
            SurfaceTexture::new(physical_size.width, physical_size.height, &window);

        let pixels = Pixels::new(
            cols * PIXELS_PER_CELL,
            rows * PIXELS_PER_CELL,
            surface_texture,
        )
        .unwrap();

        (
            Gfx {
                window,
                pixels,
                width: cols * PIXELS_PER_CELL,
                height: rows * PIXELS_PER_CELL,
            },
            event_loop,
        )
    }

    pub fn render(&mut self) {
        self.pixels.render().unwrap();
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }

    pub fn flow4_display(&mut self, board: &Board) {
        let n_colors = num_colors(board) as u8;
        let frame = self.pixels.frame_mut();

        let W = self.width as usize;

        let white = &[255, 255, 255, 255];
        frame.fill(0);

        for i in 0..board.len() {
            let c = board[i];
            match c {
                Cell::Empty {} => {}
                _ => {
                    // let k = 7; // must be coprime to n_colors
                    // let hue = (k * c.color() % n_colors) as f64 * 360.0 / n_colors as f64;
                    // let rgb = hsv_to_rgb(hue, 0.7, 0.9);
                    let color;
                    if c.color() < 10 {
                        let hue = (c.color() % n_colors) as f64 * 30.0 as f64;
                        let rgb = hsv_to_rgb(hue, 1.0, 1.0);
                        color = [rgb.0, rgb.1, rgb.2, 255];
                    } else {
                        let mut decoded = [0; 3];
                        hex::decode_to_slice(COLORS[c.color() as usize - 10], &mut decoded)
                            .expect("Decoding failed");
                        color = [decoded[0], decoded[1], decoded[2], 255];
                    }

                    let topleft = ((i * PPC) / self.width as usize * PPC * self.width as usize
                        + (i * PPC) % self.width as usize)
                        * 4;

                    frame[topleft..topleft + 12].copy_from_slice(&color.repeat(PPC));

                    frame[topleft + (self.width as usize) * 4
                        ..topleft + (self.width as usize) * 4 + 12]
                        .copy_from_slice(&color.repeat(PPC));

                    frame[topleft + (self.width as usize) * 2 * 4
                        ..topleft + (self.width as usize) * 2 * 4 + 12]
                        .copy_from_slice(&color.repeat(PPC));

                    frame[topleft + 4 + (self.width as usize) * 4
                        ..topleft + 4 + (self.width as usize) * 4 + 4]
                        .copy_from_slice(white);

                    let orientation = orientation(board, i);
                    if orientation & 1 == 1 {
                        frame[topleft + 4..topleft + 4 + 4].copy_from_slice(white);
                    }

                    if orientation & 2 == 2 {
                        frame[topleft + (self.width as usize) * 4
                            ..topleft + (self.width as usize) * 4 + 4]
                            .copy_from_slice(white);
                    }

                    if orientation & 4 == 4 {
                        frame[topleft + 4 + (self.width as usize) * 2 * 4
                            ..topleft + 4 + (self.width as usize) * 2 * 4 + 4]
                            .copy_from_slice(white);
                    }

                    if orientation & 8 == 8 {
                        frame[topleft + 8 + (self.width as usize) * 4
                            ..topleft + 8 + (self.width as usize) * 4 + 4]
                            .copy_from_slice(white);
                    }

                    if c.is_head() {
                        frame[topleft..topleft + 12].copy_from_slice(&color.repeat(PPC));

                        frame[topleft + (self.width as usize) * 4
                            ..topleft + (self.width as usize) * 4 + 12]
                            .copy_from_slice(&color.repeat(PPC));

                        frame[topleft + (self.width as usize) * 2 * 4
                            ..topleft + (self.width as usize) * 2 * 4 + 12]
                            .copy_from_slice(&color.repeat(PPC));
                        frame[topleft + 4 + (self.width as usize) * 4
                            ..topleft + 4 + (self.width as usize) * 4 + 4]
                            .copy_from_slice(white);
                    }
                }
            }
        }
        // frame[0..4].copy_from_slice(&[0, 255, 255, 255]);
    }
}

fn _rst(frame: &mut [u8]) {
    let black = [0, 0, 0, 255].repeat(frame.len() / 4);
    frame.copy_from_slice(&black)
}
