use nannou::prelude::{App, Frame, Update, PURPLE, WindowId, random_range};
use nannou::LoopMode;
use std::time::Duration;
use crate::rectangle::{Direction, Rectangle, Move};

mod rectangle;
mod movement;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct Model {
    _window: WindowId,
    rectangles: Vec<Rectangle>
}

impl Model {
    fn builder() -> ModelBuilder {
        ModelBuilder::new()
    }
}

struct ModelBuilder {
    rect_height: Option<f32>,
    rect_width: Option<f32>,
    number_rects: Option<u16>
}

impl ModelBuilder {
    fn new() -> ModelBuilder {
        Self { rect_height: None, rect_width: None, number_rects: None }
    }
    fn with_rect_width(mut self, width: f32) -> ModelBuilder {
        self.rect_width = Some(width);
        self
    }
    fn with_rect_height(mut self, height: f32) -> ModelBuilder {
        self.rect_height = Some(height);
        self
    }
    fn with_number_rects(mut self, number_rects: u16) -> ModelBuilder {
        self.number_rects = Some(number_rects);
        self
    }
    fn build(self, _window: WindowId, app: &App) -> Model {
        Model {
            _window,
            rectangles: (0..self.number_rects.unwrap_or(5)).map(
                |count| Rectangle::builder()
                    .with_x(random_range(0.0, app.window_rect().w()))
                    .with_y(random_range(0.0, app.window_rect().h()))
                    .with_x_direction(match count % 4 { 0 | 1 => Direction::POSITIVE, _ => Direction::NEGATIVE})
                    .with_y_direction(match count % 4 { 0 | 2 => Direction::POSITIVE, _ => Direction::NEGATIVE})
                    .build(app)
            ).collect()
        }
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Rate { update_interval: Duration::from_millis(10)});
    let _window = app
        .new_window()
        .with_dimensions(620, 520)
        .with_title("Chantry's Test")
        .view(view)
        .build()
        .unwrap();

    Model::builder()
        .with_rect_height(10.0)
        .with_rect_width(10.0)
        .with_number_rects(600)
        .build(_window, app)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rectangles.iter_mut().for_each(|rect| rect.step());
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(PURPLE);
    let win = app.window_rect();
    model.rectangles.iter().for_each(|rect| {
        draw.rect()
            .x_y(win.left() + rect.width / 2.0 + rect.x, win.top() - rect.height / 2.0 - rect.y)
            .w(rect.width)
            .h(rect.height)
            .hsv(0.4, 1.0, 1.0)
            .color(rect.color);
    });
    draw.to_frame(app, &frame).unwrap();
    frame
}