use crate::utils;
use super::{Point, SVG, Dimensions, html_element};


pub struct Path {
    pub uuid: String,
    points: Vec<Point>,
    starting_points: Vec<usize>,
    active_point: Option<usize>,
}


impl Path {
    ///
    pub fn new() -> Path {
        Path {
            uuid: utils::gen_uuid(),
            points: vec![],
            starting_points: vec![],
            active_point: None,
        }
    }

    ///
    pub fn from_points(points: Vec<Point>) -> Result<Path, &'static str> {
        if let Some(_) = points.first() {
            return Ok(Path {
                uuid: utils::gen_uuid(),
                points,
                starting_points: vec![0],
                active_point: None,
            });
        }

        Err("Expected at least one point to create a Path")
    }

    ///
    pub fn add_nested_path(&mut self, points: Vec<Point>) -> &Path {
        let starting_point = self.points.len();
        self.points.extend(points);
        self.starting_points.push(starting_point);
        self
    }

    ///
    pub fn get_point(&mut self, i: usize) -> Option<&mut Point> {
        self.points.get_mut(i)
    }

    ///
    pub fn select_point(&mut self, i: usize) {
        self.active_point = Some(i);
    }

    ///
    pub fn box_ptr(self) -> Box<Self> {
        Box::new(self)
    }
}

impl SVG for Path {
    ///
    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    ///
    fn to_html(&self) -> String {
        let path = self.points.iter()
            .enumerate()
            .map(|(i, point)| {
                let p_type = if self.starting_points.contains(&i) { "M" } else { "L" };
                format!("{} {} {}", p_type, point.x, point.y)
            })
            .collect::<Vec<String>>()
            .join(" ");

        html_element("path", vec![("d", path)])
    }

    ///
    fn move_to(&mut self, x: f64, y: f64) {
        let origin = self.points.first_mut().unwrap();

        let dx = x - origin.x;
        let dy = y - origin.y;

        for point in &mut self.points {
            point.x += dx;
            point.y += dy;
        }
    }

    ///
    fn resize(&mut self, dim: Dimensions) -> Result<(), String> {
        if let Dimensions::IndexPosition(i, x, y) = dim {
            if let Some(mut point) = self.get_point(i) {
                point.x = x;
                point.y = y;
                Ok(())
            } else {
                Err(format!("Cannot resize Path because point does not exist at index {}", i))
            }
        } else {
            Err(format!("Cannot resize Path with dimensions {}", dim))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    /// Verify the `to_html` function outputs an svg string reflecting the current state of the path.
    #[test]
    fn to_html() {
        let mut path = Path::from_points(vec![
            Point{ x: 0.0, y: 5.0 },
            Point{ x: 2.5, y: 0.0 },
            Point{ x: -2.5, y: 0.0 },
            Point{ x: 0.0, y: 5.0 },
        ]).unwrap();
        assert_eq!(path.to_html(),"<path d=\"M 0 5 L 2.5 0 L -2.5 0 L 0 5\"></path>");
        
        path.move_to(1.0, 4.5);
        assert_eq!(path.to_html(),"<path d=\"M 1 4.5 L 3.5 -0.5 L -1.5 -0.5 L 1 4.5\"></path>");
        
        path.add_nested_path(vec![
            Point{ x: 4.1, y: 2.3},
            Point{ x: 3.0, y: 6.1},
            Point{ x: 7.4, y: -3.0},
        ]);
        assert_eq!(path.to_html(),"<path d=\"M 1 4.5 L 3.5 -0.5 L -1.5 -0.5 L 1 4.5 M 4.1 2.3 L 3 6.1 L 7.4 -3\"></path>");

        path.resize(Dimensions::IndexPosition(4, 4.2, 1.7)).unwrap();
        assert_eq!(path.to_html(),"<path d=\"M 1 4.5 L 3.5 -0.5 L -1.5 -0.5 L 1 4.5 M 4.2 1.7 L 3 6.1 L 7.4 -3\"></path>");
    }

    /// Verify the `add_nested_path` function adds points to the existing list, and records the starting point indices.
    #[test]
    fn add_nested_path() {
        let mut path = Path::from_points(vec![
            Point{ x: 1.0, y: 2.0 },
            Point{ x: 2.0, y: 1.0 },
        ]).unwrap();
        assert_eq!(path.starting_points, vec![0]);
            
        path.add_nested_path(vec![
            Point{ x: 3.0, y: 3.0 },
            Point{ x: 4.0, y: 5.0 },
            Point{ x: 5.0, y: 4.0 },
        ]);
        assert_eq!(path.points, vec![
            Point{ x: 1.0, y: 2.0 },
            Point{ x: 2.0, y: 1.0 },
            Point{ x: 3.0, y: 3.0 },
            Point{ x: 4.0, y: 5.0 },
            Point{ x: 5.0, y: 4.0 },
        ]);
        assert_eq!(path.starting_points, vec![0, 2]);
    }

    /// Verify the `move_to` function changes the coordinates of all points in the path.
    #[test]
    fn move_to() {
        let mut path = Path::from_points(vec![
            Point{ x: 0.0, y: 5.0 },
            Point{ x: 2.5, y: 0.0 },
            Point{ x: -2.5, y: 0.0 },
            Point{ x: 0.0, y: 5.0 },
        ]).unwrap();

        path.move_to(4.2, -1.0);
        assert_eq!(path.points, vec![
            Point{ x: 4.2, y: -1.0 },
            Point{ x: 6.7, y: -6.0 },
            Point{ x: 1.7, y: -6.0 },
            Point{ x: 4.2, y: -1.0 },
        ])
    }


    /// Verify the `resize` function changes the position of a given point in the path, and the other points remain.
    #[test]
    fn resize() {
        let mut path = Path::from_points(vec![
            Point{ x: 0.0, y: 0.0 },
            Point{ x: 1.0, y: 0.0 },
            Point{ x: 1.0, y: 1.0 },
            Point{ x: 0.0, y: 1.0 },
            Point{ x: 0.0, y: 0.0 },
        ]).unwrap();

        // Verify resizing with a valid dimension
        path.resize(Dimensions::IndexPosition(0, 0.1, 1.9)).unwrap();
        path.resize(Dimensions::IndexPosition(2, 0.2, 1.8)).unwrap();
        path.resize(Dimensions::IndexPosition(4, 0.3, 1.7)).unwrap();
        assert_eq!(path.points, vec![
            Point{ x: 0.1, y: 1.9 },
            Point{ x: 1.0, y: 0.0 },
            Point{ x: 0.2, y: 1.8 },
            Point{ x: 0.0, y: 1.0 },
            Point{ x: 0.3, y: 1.7 },
        ]);

        // Verify attempting to resize with a valid dimension but invalid index
        let err = path.resize(Dimensions::IndexPosition(5, 1.0, 2.0)).unwrap_err();
        assert_eq!(err, "Cannot resize Path because point does not exist at index 5");
        assert_eq!(path.points, vec![
            Point{ x: 0.1, y: 1.9 },
            Point{ x: 1.0, y: 0.0 },
            Point{ x: 0.2, y: 1.8 },
            Point{ x: 0.0, y: 1.0 },
            Point{ x: 0.3, y: 1.7 },
        ]);

        // Verify attempting to resize with invalid Single dimensions
        let err = path.resize(Dimensions::Single(1.0)).unwrap_err();
        assert_eq!(err, "Cannot resize Path with dimensions Single(1.0)");
        assert_eq!(path.points, vec![
            Point{ x: 0.1, y: 1.9 },
            Point{ x: 1.0, y: 0.0 },
            Point{ x: 0.2, y: 1.8 },
            Point{ x: 0.0, y: 1.0 },
            Point{ x: 0.3, y: 1.7 },
        ]);

        // Verify attempting to resize with invalid Double dimensions
        let err = path.resize(Dimensions::Double(1.0, 2.0)).unwrap_err();
        assert_eq!(err, "Cannot resize Path with dimensions Double(1.0, 2.0)");
        assert_eq!(path.points, vec![
            Point{ x: 0.1, y: 1.9 },
            Point{ x: 1.0, y: 0.0 },
            Point{ x: 0.2, y: 1.8 },
            Point{ x: 0.0, y: 1.0 },
            Point{ x: 0.3, y: 1.7 },
        ]);
    }
}