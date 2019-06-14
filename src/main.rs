use nannou::prelude::{App, Frame, Update, PURPLE, WindowId, random_range, Hsla};
use nannou::LoopMode;
use std::time::Duration;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
#[derive(Debug)]
enum Direction {
    POSITIVE, NEGATIVE
}

#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    max_x: f32,
    max_y: f32,
    direction_x: Direction,
    direction_y: Direction,
    color: Hsla<f32>
}

#[derive(Debug)]
struct Model {
    _window: WindowId,
    rectangles: Vec<Rectangle>
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
    fn with_rect_width(&mut self, width: f32) -> &mut ModelBuilder {
        self.rect_width = Some(width);
        self
    }
    fn with_rect_height(&mut self, height: f32) -> &mut ModelBuilder {
        self.rect_height = Some(height);
        self
    }
    fn with_number_rects(&mut self, number_rects: u16) -> &mut ModelBuilder {
        self.number_rects = Some(number_rects);
        self
    }
    fn build(&mut self, _window: WindowId, app: &App) -> Model {
        let rect_width = *self.rect_width.get_or_insert(25.0);
        let rect_height = *self.rect_height.get_or_insert(25.0);
        Model {
            _window,
            rectangles: (0..*self.number_rects.get_or_insert(5)).map(|x| Rectangle {
                x: random_range(0.0, app.window_rect().w()),
                y: random_range(0.0, app.window_rect().h()),
                max_x: app.window_rect().w() as f32 - rect_width,
                max_y: app.window_rect().h() as f32 - rect_height,
                height: rect_height,
                width: rect_width,
                direction_x: match x % 4 { 0 | 1 => Direction::POSITIVE, _ => Direction::NEGATIVE},
                direction_y: match x % 4 { 0 | 2 => Direction::POSITIVE, _ => Direction::NEGATIVE},
                color: Hsla::new(
                    random_range(-180.0, 180.0).into(),
                    1.0,
                    0.5,
                    random_range(0.2, 1.0)
                )
            }).collect()
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

    ModelBuilder::new()
        .with_rect_height(20.0)
        .with_rect_width(20.0)
        .with_number_rects(160)
        .build(_window, app)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rectangles.iter_mut().for_each(|rect| {
        match rect.direction_x {
            Direction::POSITIVE => {
                let mut next_x = rect.x + 1.0;
                if next_x > rect.max_x {
                    rect.direction_x = Direction::NEGATIVE;
                    next_x = rect.max_x + rect.max_x - next_x;
                }
                rect.x = next_x;
            },
            Direction::NEGATIVE => {
                let mut next_x = rect.x - 1.0;
                if next_x < 0.0 {
                    rect.direction_x = Direction::POSITIVE;
                    next_x = -1.0 * next_x;
                }
                rect.x = next_x;
            }
        }

        match &rect.direction_y {
            Direction::POSITIVE => {
                let mut next_y = rect.y + 1.5;
                if next_y > rect.max_y {
                    rect.direction_y = Direction::NEGATIVE;
                    next_y = rect.max_y + rect.max_y - next_y;
                }
                rect.y = next_y;
            },
            Direction::NEGATIVE => {
                let mut next_y = rect.y - 1.5;
                if next_y < 0.0 {
                    rect.direction_y = Direction::POSITIVE;
                    next_y = -1.0 * next_y;
                }
                rect.y = next_y;
            }
        }
    });
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