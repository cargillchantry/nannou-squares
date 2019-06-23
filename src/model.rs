use nannou::prelude::{WindowId, random_range};
use nannou::App;
use crate::rectangle::{Direction, Rectangle};
use nannou::rand::random;

#[derive(Debug)]
pub struct Model {
    pub _window: WindowId,
    pub rectangles: Vec<Rectangle>
}

impl Model {
    pub fn builder() -> ModelBuilder {
        ModelBuilder::new()
    }
}

pub struct ModelBuilder {
    rect_height: Option<f32>,
    rect_width: Option<f32>,
    number_rects: Option<u16>
}

impl ModelBuilder {
    fn new() -> ModelBuilder {
        Self { rect_height: None, rect_width: None, number_rects: None }
    }
    pub fn with_rect_width(mut self, width: f32) -> ModelBuilder {
        self.rect_width = Some(width);
        self
    }
    pub fn with_rect_height(mut self, height: f32) -> ModelBuilder {
        self.rect_height = Some(height);
        self
    }
    pub fn with_number_rects(mut self, number_rects: u16) -> ModelBuilder {
        self.number_rects = Some(number_rects);
        self
    }
    pub fn build(self, _window: WindowId, app: &App) -> Model {
        let rect_width = self.rect_width.unwrap_or(10.0);
        let rect_height = self.rect_height.unwrap_or(10.0);
        let bound_x = (app.window_rect().w() - rect_width)/2.0;
        let bound_y = (app.window_rect().h() - rect_height)/2.0;
        Model {
            _window,
            rectangles: (0..self.number_rects.unwrap_or(5)).map(
                |_| Rectangle::builder()
                    .with_width(rect_width)
                    .with_height(rect_height)
                    .with_x(random_range(-bound_x, bound_x))
                    .with_y(random_range(-bound_y, bound_y))
                    .with_x_direction(if random() { Direction::POSITIVE } else { Direction::NEGATIVE })
                    .with_y_direction(if random() { Direction::POSITIVE } else { Direction::NEGATIVE })
                    .build(app)
            ).collect()
        }
    }
}