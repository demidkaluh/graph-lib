#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Clone)]
enum Color {
    RED = 0xFF0000,
    ORANGE = 0xFFAA00,
    YELLOW = 0xFFFF00,
    GREEN = 0x00FF00,
    CYAN = 0x00AAFF,
    BLUE = 0x0000FF,
    MAGENTA = 0xAA00FF,
}

struct Plot {
    data: Vec<Vec<Color>>,
}

impl Plot {
    fn new(x: usize, y: usize) -> Self {
        Self {
            data: vec![vec![x], 10],
        }
    }

    fn set(&mut self, x: usize, y: usize, color: Color) {
        self.data[x][y] = color;
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    fn get(&self, x: usize, y: usize) -> Color {
        self.data[x][y].clone()
    }
}

fn rainbow_bg(plot: &mut Plot) {
    for y in 0..plot.height() {
        for x in 0..plot.width() {
            match (x + y) % 7 {
                0 => plot.set(x, y, Color::RED),
                1 => plot.set(x, y, Color::ORANGE),
                2 => plot.set(x, y, Color::YELLOW),
                3 => plot.set(x, y, Color::GREEN),
                4 => plot.set(x, y, Color::CYAN),
                5 => plot.set(x, y, Color::BLUE),
                6 => plot.set(x, y, Color::MAGENTA),
                _ => (),
            }
        }
    }
}

fn save_as_ppm(file_path: &str, plot: &Plot, width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    write!(file, "P6\n{} {} 255\n", width, height)?;
    for y in 0..height {
        for x in 0..width {
            let pixel = plot.get(x, y) as u32;
            let color = [
                ((pixel >> 8 * 2) & 0xFF) as u8,
                ((pixel >> 8 * 1) & 0xFF) as u8,
                ((pixel >> 8 * 0) & 0xFF) as u8,
            ];
            file.write(&color)?;
        }
    }

    Ok(())
}

fn draw_hollow_rectangle(rect: &figure::Rectangle) {}
fn draw_filled_rectangle(rect: &figure::Rectangle) {}

pub mod figure {
    pub struct Rectangle {
        pos: (u32, u32), // left-bottom edge
        width: u32,
        length: u32,
    }

    impl Rectangle {
        pub fn new(pos: (u32, u32), width: u32, length: u32) -> Self {
            Self { pos, width, length }
        }

        pub fn area(&self) -> u32 {
            self.width * self.length
        }

        pub fn can_hold(&self, rect: &Rectangle) -> bool {
            let mut res = true;
            res &= self.pos.0 <= rect.pos.0;
            res &= self.pos.1 <= rect.pos.1;
            res &= self.pos.0 + self.width >= rect.pos.0 + rect.width;
            res &= self.pos.1 + self.length >= rect.pos.1 + rect.length;

            return res;
        }
    }

    pub struct Circle {
        pos: (u32, u32),
        radius: u32,
    }

    impl Circle {
        pub fn new(pos: (u32, u32), radius: u32) -> Self {
            Self { pos, radius }
        }

        pub fn area(&self) -> f32 {
            std::f32::consts::PI * ((self.radius * self.radius) as f32)
        }

        pub fn can_hold(&self, circle: &Circle) -> bool {
            let x_diff_sq = ((circle.pos.0 - self.pos.0) as f32).powi(2);
            let y_diff_sq = ((circle.pos.1 - self.pos.1) as f32).powi(2);
            (f32::sqrt((x_diff_sq + y_diff_sq) as f32) - (circle.radius as f32)).abs()
                <= (self.radius as f32)
        }
    }
}

fn main() {
    const HEIGHT: usize = 50;
    const WIDTH: usize = 50;
    let path = "smth.ppm";

    rainbow_bg(&mut plot, WIDTH, HEIGHT);
    save_as_ppm(path, &plot, WIDTH, HEIGHT);
}
