use crate::utils;
use super::{Point, SVG, Dimensions, html_element};


pub struct Circle {
    uuid: String,
    origin: Point,
    radius: f64,
}

impl Circle {
    ///
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            uuid: utils::gen_uuid(),
            origin: Point { x, y },
            radius
        }
    }

    ///
    pub fn box_ptr(self) -> Box<Self> {
        Box::new(self)
    }
}

impl SVG for Circle {
    ///
    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    ///
    fn to_html(&self) -> String {
        html_element("circle", vec![
            ("cx", self.origin.x.to_string()),
            ("cy", self.origin.y.to_string()),
            ("r", self.radius.to_string()),
        ])
    }

    ///
    fn move_to(&mut self, x: f64, y: f64) {
        self.origin.x = x;
        self.origin.y = y;
    }

    ///
    fn resize(&mut self, dim: Dimensions) -> Result<(), String> {
        if let Dimensions::Single(r) = dim {
            self.radius = r;
            Ok(())
        } else {
            Err(format!("Cannot resize Circle with dimensions {}", dim))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Verify the `to_html` function outputs an svg string reflecting the current state of the circle.
    #[test]
    fn to_html() {
        let mut circle = Circle::new(0.0, 1.5, 5.2);
        assert_eq!(circle.to_html(),"<circle cx=\"0\" cy=\"1.5\" r=\"5.2\"></circle>");
        
        circle.move_to(-1.5, 4.0);
        assert_eq!(circle.to_html(),"<circle cx=\"-1.5\" cy=\"4\" r=\"5.2\"></circle>");
        
        circle.resize(Dimensions::Single(3.0)).unwrap();
        assert_eq!(circle.to_html(),"<circle cx=\"-1.5\" cy=\"4\" r=\"3\"></circle>");
    }

    /// Verify the `move_to` function changes the origin of the circle, and the radius stays the same.
    #[test]
    fn move_to() {
        let mut circle = Circle::new(5.0, 10.0, 20.0);
        assert_eq!(circle.origin, Point{x: 5.0, y: 10.0});
        assert_eq!(circle.radius, 20.0);

        circle.move_to(12.0, -16.0);
        assert_eq!(circle.origin, Point{x: 12.0, y: -16.0});
        assert_eq!(circle.radius, 20.0);
    }

    /// Verify the `resize` function changes the radius of the circle, and the origin stays the same.
    #[test]
    fn resize() {
        let mut circle = Circle::new(5.0, 10.0, 20.0);
        assert_eq!(circle.origin, Point{x: 5.0, y: 10.0});
        assert_eq!(circle.radius, 20.0);

        // Verify resizing with a valid dimension
        circle.resize(Dimensions::Single(3.5)).unwrap();
        assert_eq!(circle.origin, Point{x: 5.0, y: 10.0});
        assert_eq!(circle.radius, 3.5);

        // Verify attempting to resize with invalid Double dimensions
        let err = circle.resize(Dimensions::Double(1.0, 2.0)).unwrap_err();
        assert_eq!(err, "Cannot resize Circle with dimensions Double(1.0, 2.0)");
        assert_eq!(circle.origin, Point{x: 5.0, y: 10.0});
        assert_eq!(circle.radius, 3.5);

        // Verify attempting to resize with invalid IndexPosition dimensions
        let err = circle.resize(Dimensions::IndexPosition(3, 1.0, 2.0)).unwrap_err();
        assert_eq!(err, "Cannot resize Circle with dimensions IndexPosition(3, 1.0, 2.0)");
        assert_eq!(circle.origin, Point{x: 5.0, y: 10.0});
        assert_eq!(circle.radius, 3.5);
    }
}