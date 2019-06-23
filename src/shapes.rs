use nannou::prelude::app::Draw;

#[derive(Debug)]
pub enum Direction {
    POSITIVE, NEGATIVE
}

pub trait Move {
    fn step(&mut self);
}

pub trait Sketch {
    fn sketch(&self, draw: &Draw);
}