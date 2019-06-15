pub use crate::movement::{Direction, Move};
use nannou::prelude::Hsla;

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub direction_x: Direction,
    pub direction_y: Direction,
    pub color: Hsla<f32>
}

impl Move for Rectangle {
    fn step(&mut self) -> () {
        match self.direction_x {
            Direction::POSITIVE => {
                let mut next_x = self.x + 1.0;
                if next_x > self.max_x {
                    self.direction_x = Direction::NEGATIVE;
                    next_x = self.max_x + self.max_x - next_x;
                }
                self.x = next_x;
            },
            Direction::NEGATIVE => {
                let mut next_x = self.x - 1.0;
                if next_x < 0.0 {
                    self.direction_x = Direction::POSITIVE;
                    next_x = -1.0 * next_x;
                }
                self.x = next_x;
            }
        }

        match &self.direction_y {
            Direction::POSITIVE => {
                let mut next_y = self.y + 1.5;
                if next_y > self.max_y {
                    self.direction_y = Direction::NEGATIVE;
                    next_y = self.max_y + self.max_y - next_y;
                }
                self.y = next_y;
            },
            Direction::NEGATIVE => {
                let mut next_y = self.y - 1.5;
                if next_y < 0.0 {
                    self.direction_y = Direction::POSITIVE;
                    next_y = -1.0 * next_y;
                }
                self.y = next_y;
            }
        }
    }
}