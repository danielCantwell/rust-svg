use crate::utils;
use super::{Point, SVG, Dimensions, html_element};


pub struct Rect {
    pub uuid: String,
    origin: Point,
    width: f64,
    height: f64,
}

impl Rect {
    ///
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rect {
        Rect {
            uuid: utils::gen_uuid(),
            origin: Point { x, y },
            width,
            height
        }
    }

    ///
    pub fn box_ptr(self) -> Box<Self> {
        Box::new(self)
    }
}

impl SVG for Rect {
    ///
    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    ///
    fn to_html(&self) -> String {
        html_element("rect", vec![
            ("x", self.origin.x.to_string()),
            ("y", self.origin.y.to_string()),
            ("width", self.width.to_string()),
            ("height", self.height.to_string()),
        ])
    }

    ///
    fn move_to(&mut self, x: f64, y: f64) {
        self.origin.x = x;
        self.origin.y = y;
    }
    
    ///
    fn resize(&mut self, dim: Dimensions) -> Result<(), String> {
        if let Dimensions::Double(width, height) = dim {
            self.width = width;
            self.height = height;
            Ok(())
        } else {
            Err(format!("Cannot resize Rect with dimensions {}", dim))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Verify the `to_html` function outputs an svg string reflecting the current state of the rect.
    #[test]
    fn to_html() {
        let mut rect = Rect::new(10.0, -3.0, 4.3, 2.0);
        assert_eq!(rect.to_html(),"<rect x=\"10\" y=\"-3\" width=\"4.3\" height=\"2\"></rect>");
        
        rect.move_to(-3.1, 4.0);
        assert_eq!(rect.to_html(),"<rect x=\"-3.1\" y=\"4\" width=\"4.3\" height=\"2\"></rect>");
        
        rect.resize(Dimensions::Double(5.0, 0.5)).unwrap();
        assert_eq!(rect.to_html(),"<rect x=\"-3.1\" y=\"4\" width=\"5\" height=\"0.5\"></rect>");
    }

    /// Verify the `move_to` function changes the origin of the rect, and the dimensions stay the same.
    #[test]
    fn move_to() {
        let mut rect = Rect::new(0.0, 0.0, 20.0, 10.0);
        assert_eq!(rect.origin, Point{x: 0.0, y: 0.0});
        assert_eq!(rect.width, 20.0);
        assert_eq!(rect.height, 10.0);

        rect.move_to(2.5, -4.0);
        assert_eq!(rect.origin, Point{x: 2.5, y: -4.0});
        assert_eq!(rect.width, 20.0);
        assert_eq!(rect.height, 10.0);
    }


    /// Verify the `resize` function changes the width and height of the rect, and the origin stays the same.
    #[test]
    fn resize() {
        let mut rect = Rect::new(0.0, 2.5, 5.0, 10.0);
        assert_eq!(rect.origin, Point{x: 0.0, y: 2.5});
        assert_eq!(rect.width, 5.0);
        assert_eq!(rect.height, 10.0);

        // Verify resizing with a valid dimension
        rect.resize(Dimensions::Double(6.1, 4.2)).unwrap();
        assert_eq!(rect.origin, Point{x: 0.0, y: 2.5});
        assert_eq!(rect.width, 6.1);
        assert_eq!(rect.height, 4.2);

        // Verify attempting to resize with invalid Single dimensions
        let err = rect.resize(Dimensions::Single(1.0)).unwrap_err();
        assert_eq!(err, "Cannot resize Rect with dimensions Single(1.0)");
        assert_eq!(rect.origin, Point{x: 0.0, y: 2.5});
        assert_eq!(rect.width, 6.1);
        assert_eq!(rect.height, 4.2);

        // Verify attempting to resize with invalid IndexPosition dimensions
        let err = rect.resize(Dimensions::IndexPosition(3, 1.0, 2.0)).unwrap_err();
        assert_eq!(err, "Cannot resize Rect with dimensions IndexPosition(3, 1.0, 2.0)");
        assert_eq!(rect.origin, Point{x: 0.0, y: 2.5});
        assert_eq!(rect.width, 6.1);
        assert_eq!(rect.height, 4.2);
    }
}