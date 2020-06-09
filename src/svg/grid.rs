use std::collections::HashMap;

use crate::utils;
use super::{SVG, Dimensions, Group};


pub enum CoordinateSystem {
    TopLeftDownRight, // Center (x,y) is top-left. Positive y is down. Positive x is right.
    BottomLeftUpRight, // Center (x,y) is bottom-left. Positive y is up. Positive x is right.
    MidMidUpRight, // Center (x,y) is absolute center. Positive y is up. Positive x is right.
    MidMidDownRight, // Center (x, y) is absolute center. Positive y is down. Positive x is right.
}


/// Stores shapes and handles user interaction to make modifications
pub struct Grid {
    uuid: String,
    width: f64,
    height: f64,
    coordinate_system: CoordinateSystem,
    view_box: String,
    groups: HashMap<String, Group>,
    shapes: HashMap<String, usize>
}

impl Grid {
    ///
    pub fn new(c : CoordinateSystem) -> Grid {
        let mut grid = Grid {
            uuid: utils::gen_uuid(),
            width: 1000.0,
            height: 1000.0,
            coordinate_system: c,
            view_box: String::new(),
            groups: HashMap::new(),
            shapes: HashMap::new(),
        };

        grid.view_box = grid.init_view_box();
        grid.groups.insert(String::from("shapes"), Group::new("shapes"));
        grid
    }

    ///
    fn init_view_box(&self) -> String {
        match self.coordinate_system {
            CoordinateSystem::TopLeftDownRight => {
                return format!("0 0 {} {}", self.width, self.height);
            },
            CoordinateSystem::BottomLeftUpRight => {
                return format!("0 {} {} {}", self.height, self.width, self.height);
            },
            CoordinateSystem::MidMidDownRight | CoordinateSystem::MidMidUpRight => {
                return format!("{} {} {} {}", self.width / 2.0, self.height / 2.0, self.width, self.height);
            },
        }
    }

    ///
    fn transform_coordinates(&self, x: f64, y: f64, from: &CoordinateSystem) -> (f64, f64) {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;

        let mut result_x = x;
        let mut result_y = y;

        match (from, &self.coordinate_system) {
            (CoordinateSystem::TopLeftDownRight, CoordinateSystem::MidMidDownRight) => {
                result_x = x - half_width;
                result_y = y - half_height;
            },
            (CoordinateSystem::TopLeftDownRight, CoordinateSystem::MidMidUpRight) => {
                result_x = x - half_width;
                result_y = half_height - y;
            },
            _ => panic!("Support for other transformations has not been implemented."),
        }

        (result_x, result_y)
    }

    /// Grid should own the group.
    pub fn add_group(&mut self, group: Group) -> &Group {
        let key = group.get_name();
        self.groups.insert(key.clone(), group);
        self.groups.get(&key).unwrap()
    }

    /// Group should own the shape, Grid should reference the shape;
    pub fn add_shape(&mut self, shape: Box<dyn SVG>) -> usize {
        let id = shape.get_uuid();
        let shapes_group = self.groups.get_mut("shapes").unwrap();

        let shape_idx = shapes_group.add_shape(shape);
        self.shapes.insert(id, shape_idx);

        shape_idx
    }

    /// 
    pub fn get_shape(&self, i: usize) -> Option<&Box<dyn SVG>> {
        self.groups.get("shapes").unwrap().get_shape(i)
    }

    /// 
    pub fn get_shape_mut(&mut self, i: usize) -> Option<&mut Box<dyn SVG>> {
        self.groups.get_mut("shapes").unwrap().get_shape_mut(i)
    }
}

impl SVG for Grid {
    ///
    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }
    
    ///
    fn to_html(&self) -> String {
        let groups_str = self.groups.values()
            .map(|group| group.to_html())
            .collect::<Vec<String>>()
            .join("\n");

        format!("<svg viewBox=\"{}\">\n{}\n</svg>", self.view_box, groups_str)
    }

    
    fn move_to(&mut self, _: f64, _: f64) { unimplemented!() }
    fn resize(&mut self, _: Dimensions) -> Result<(), String> { unimplemented!() }
}