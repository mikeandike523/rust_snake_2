use std::cmp::min;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::appstate::AppState;

pub struct Grid {
    pub(crate) rows: i32,
    pub(crate) cols: i32,
    pixels: Vec<bool>,
}

impl Grid {
    pub fn new(rows: i32, cols: i32) -> Grid {
        let num_pixels = rows * cols;
        let mut pixels: Vec<bool> = Vec::new();
        for i in 0..num_pixels {
            pixels.push(false);
        }
        Grid {
            rows,
            cols,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, value: bool) {
        let slice = self.pixels.get_mut((y * self.cols + x) as usize).expect("Could not set grid pixel.");
        *slice = value;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> bool {
        let slice = self.pixels.get((y * self.cols + x) as usize).expect("Could not get grid pixel.");
        *slice
    }

    pub fn clear(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                self.set_pixel(x, y, false);
            }
        }
    }

    pub fn render_to(&self, canvas: &mut WindowCanvas, appstate: &AppState) {
        let w = appstate.w;
        let h = appstate.h;
        let rows = self.rows;
        let cols = self.cols;

        let square_size = min(w / cols, h / rows);

        // Will be off-center by 0.5 pixels if dimensions are even

        let margin_x = (w - square_size * cols) / 2;
        let margin_y = (h - square_size * rows) / 2;


        for row in 0..rows {
            for col in 0..cols {
                if self.get_pixel(col, row) {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }

                canvas.fill_rect(
                    Rect::new(margin_x + col * square_size,
                              margin_y + row * square_size,
                              square_size as u32,
                              square_size as u32))
                    .expect("Unknown error when rendering grid to canvas.");
            }
        }
    }

    pub fn render_pixel(&self, canvas: &mut WindowCanvas, appstate: &AppState, x: i32, y: i32, color: Color) {
        let w = appstate.w;
        let h = appstate.h;
        let rows = self.rows;
        let cols = self.cols;

        let square_size = min(w / cols, h / rows);

        // Will be off-center by 0.5 pixels if dimensions are even

        let margin_x = (w - square_size * cols) / 2;
        let margin_y = (h - square_size * rows) / 2;

        let row = y;
        let col = x;

        canvas.set_draw_color(color);

        canvas.fill_rect(
            Rect::new(margin_x + col * square_size,
                      margin_y + row * square_size,
                      square_size as u32,
                      square_size as u32))
            .expect("Unknown error when rendering grid to canvas.");
    }
}
