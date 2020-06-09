use crate::utils;
use super::{SVG, Dimensions};


pub struct Group {
    uuid: String,
    name: String,
    shapes: Vec<Box<dyn SVG>>,
}

impl Group {
    ///
    pub fn new(name: &str) -> Group {
        Group {
            uuid: utils::gen_uuid(),
            name: String::from(name),
            shapes: Vec::new(),
        }
    }

    ///
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Group should own the shape.
    pub fn add_shape<'a>(&mut self, shape: Box<dyn SVG>) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    ///
    pub fn get_shape_htmls(&self) -> Vec<String> {
        self.shapes.iter()
            .map(|shape| shape.to_html())
            .collect::<Vec<String>>()
    }

    ///
    pub fn get_shape(&self, i: usize) -> Option<&Box<dyn SVG>> {
        self.shapes.get(i)
    }

    ///
    pub fn get_shape_mut(&mut self, i: usize) -> Option<&mut Box<dyn SVG>> {
        self.shapes.get_mut(i)
    }
}

impl SVG for Group {
    ///
    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    ///
    fn to_html(&self) -> String {
        format!("<g name=\"{}\">\n{}\n</g>", self.name, self.get_shape_htmls().join("\n"))
    }

    fn move_to(&mut self, _: f64, _: f64) { unimplemented!() }
    fn resize(&mut self, _: Dimensions) -> Result<(), String> { unimplemented!() }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{Circle, Rect};

    /// Verify the `get_shape_htmls` function returns a list of svg strings for each shape in the group.
    #[test]
    fn shape_svgs() {
        let mut group = Group::new("test_group");
        assert_eq!(group.shapes.len(), 0);
        assert_eq!(group.get_shape_htmls().len(), 0);

        let circle = Circle::new(1.0, 2.0, 3.0);
        group.add_shape(circle.box_ptr());
        assert_eq!(group.get_shape_htmls(), vec![
            "<circle cx=\"1\" cy=\"2\" r=\"3\"></circle>"
        ]);
        
        let rect = Rect::new(0.0, 1.0, 2.0, 3.0);
        group.add_shape(rect.box_ptr());
        assert_eq!(group.get_shape_htmls(), vec![
            "<circle cx=\"1\" cy=\"2\" r=\"3\"></circle>",
            "<rect x=\"0\" y=\"1\" width=\"2\" height=\"3\"></rect>",
        ]);
    }

    /// Verify the `to_html` function outputs an svg string reflecting the current state of the group.
    #[test]
    fn to_html() {
        let mut group = Group::new("test_group");
        assert_eq!(group.to_html(), "<g name=\"test_group\">\n\n</g>");

        let circle = Circle::new(1.0, 2.0, 3.0);
        let rect = Rect::new(0.0, 1.0, 2.0, 3.0);
        group.add_shape(circle.box_ptr());
        group.add_shape(rect.box_ptr());

        let expected_html = "\
<g name=\"test_group\">
<circle cx=\"1\" cy=\"2\" r=\"3\"></circle>
<rect x=\"0\" y=\"1\" width=\"2\" height=\"3\"></rect>
</g>";
        assert_eq!(group.to_html(), expected_html);
    }
}