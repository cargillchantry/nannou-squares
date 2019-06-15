#[derive(Debug)]
pub enum Direction {
    POSITIVE, NEGATIVE
}

pub trait Move {
    fn step(&mut self) -> ();
}
