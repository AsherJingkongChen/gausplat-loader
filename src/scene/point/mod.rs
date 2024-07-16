pub mod points;

pub use points::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub color: [f64; 3],
    pub position: [f64; 3],
}
