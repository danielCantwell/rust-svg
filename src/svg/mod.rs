use std::fmt;

mod circle;
mod path;
mod rect;
mod group;
mod grid;

pub use circle::Circle;
pub use path::Path;
pub use rect::Rect;
pub use group::Group;
pub use grid::{Grid, CoordinateSystem};


#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn approx_eq(a: f64, b: f64) -> bool {
    if a == b {
        return true;
    }

    let a_abs = a.abs();
    let b_abs = b.abs();

    if a == 0.0 || b == 0.0 || (a_abs + b_abs < std::f64::MIN_POSITIVE) {
        return (a - b).abs() < std::f64::EPSILON * std::f64::MIN_POSITIVE;
    }

    let sum = a_abs + b_abs;
    let min = if sum < std::f64::MAX { sum } else { std::f64::MAX };

    (a - b).abs() / min < std::f64::EPSILON
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y)
    }
}


#[derive(Debug)]
pub enum Dimensions {
    Single(f64),
    Double(f64, f64),
    IndexPosition(usize, f64, f64),
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub trait SVG {
    fn get_uuid(&self) -> String;
    fn to_html(&self) -> String;

    fn move_to(&mut self, x: f64, y: f64);
    fn resize(&mut self, dims: Dimensions) -> Result<(), String>;
}

fn html_element(tag: &str, attrs: Vec<(&str, String)>) -> String {
    let attr_str = attrs.iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v))
        .collect::<Vec<String>>()
        .join(" ");

    format!("<{0} {1}></{0}>", tag, attr_str)
}
