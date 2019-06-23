use nannou::prelude::{App, Frame, Update, BLACK};
use nannou::LoopMode;
use std::time::Duration;
use crate::model::Model;
use crate::shapes::{Sketch, Move};

mod rectangle;
mod shapes;
mod model;

fn main() {
    nannou::app(model).update(update).run();
}


fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Rate { update_interval: Duration::from_millis(5)});
    let _window = app
        .new_window()
        .with_dimensions(1320, 720)
        .with_title("Chantry's Test")
        .view(view)
        .build()
        .unwrap();

    Model::builder()
        .with_rect_height(40.0)
        .with_rect_width(40.0)
        .with_number_rects(50)
        .build(_window, app)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rectangles.iter_mut().for_each(|rect| rect.step());
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.rectangles.iter().for_each(|rect| rect.sketch(&draw));
    draw.to_frame(app, &frame).unwrap();
    frame
}