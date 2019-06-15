pub use crate::movement::{Direction, Move};
use nannou::prelude::{Hsla, random_range};
use nannou::App;

pub struct RectangleBuilder {
    width: Option<f32>,
    height: Option<f32>,
    x: Option<f32>,
    y: Option<f32>,
    direction_x: Option<Direction>,
    direction_y: Option<Direction>
}

impl RectangleBuilder {
    fn new() -> RectangleBuilder {
        RectangleBuilder {
            width: None, height: None, x: None, y: None, direction_x: None, direction_y: None
        }
    }
    pub fn with_width(mut self, width: f32) -> RectangleBuilder {
        self.width = Some(width);
        self
    }
    pub fn with_height(mut self, height: f32) -> RectangleBuilder {
        self.height = Some(height);
        self
    }
    pub fn with_x(mut self, x: f32) -> RectangleBuilder {
        self.x = Some(x);
        self
    }
    pub fn with_y(mut self, y: f32) -> RectangleBuilder {
        self.y = Some(y);
        self
    }
    pub fn with_x_direction(mut self, direction: Direction) -> RectangleBuilder {
        self.direction_x = Some(direction);
        self
    }
    pub fn with_y_direction(mut self, direction: Direction) -> RectangleBuilder {
        self.direction_y = Some(direction);
        self
    }
    pub fn build(self, app: &App) -> Rectangle {
        let rect_width = self.width.unwrap_or(25.0);
        let rect_height = self.height.unwrap_or(25.0);
        Rectangle {
            x: self.x.unwrap_or(0.0),
            y: self.y.unwrap_or(0.0),
            max_x: app.window_rect().w() as f32 - rect_width,
            max_y: app.window_rect().h() as f32 - rect_height,
            height: rect_height,
            width: rect_width,
            direction_x: self.direction_x.unwrap_or(Direction::POSITIVE),
            direction_y: self.direction_y.unwrap_or(Direction::POSITIVE),
            color: Hsla::new(
                random_range(-180.0, 180.0).into(),
                1.0,
                0.5,
                random_range(0.2, 1.0)
            )
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub color: Hsla<f32>,
    max_x: f32,
    max_y: f32,
    direction_x: Direction,
    direction_y: Direction,
}

impl Rectangle {
    pub fn builder() -> RectangleBuilder {
        RectangleBuilder::new()
    }
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