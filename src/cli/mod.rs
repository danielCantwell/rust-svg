use std::io::prelude::*;
use std::io;

use crate::svg::{Grid, Point, SVG, Rect, Circle, Path, Dimensions};
use crate::utils;


/// Read a value from user input
pub fn read_input() -> Result<String, &'static str> {
    print!("Enter Command: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input = String::new();
    let line = io::stdin().read_line(&mut input);

    match line {
        Ok(_) => Ok(input),
        Err(_) => Err("Failed to read user input"),
    }
}


/// Parse and execute commands
pub fn execute_command(grid: &mut Grid, cmd: String) -> Result<(), String> {
    let args_iter : Vec<&str> = cmd.split_whitespace().collect();
    let root_arg = args_iter[0];
    let rest = &args_iter[1..];

    match root_arg {
        "html" => println!("{}", grid.to_html()),
        "draw" => cmd_draw(grid, rest)?,
        "move" => cmd_move(grid, rest)?,
        "resize" => cmd_resize(grid, rest)?,
        _ => {
            return Err(String::from(format!("Unable to parse command {}.", args_iter[0])));
        }
    }

    Ok(())
}


/// Handle drawing shapes
fn cmd_draw(grid: &mut Grid, args: &[&str]) -> Result<(), String> {
    match args[..] {
        [shape, x, y] => {
            let x = utils::str_to_float(x)?;
            let y = utils::str_to_float(y)?;

            let shape_idx : usize;

            match shape {
                "rect" => {
                    shape_idx = grid.add_shape(Rect::new(x, y, 0.0, 0.0).box_ptr());
                },
                "circle" => {
                    shape_idx = grid.add_shape(Circle::new(x, y, 0.0).box_ptr());
                },
                "path" => {
                    let origin = Point{x, y};
                    shape_idx = grid.add_shape(Path::from_points(vec![origin])?.box_ptr());
                },
                _ => return Err(format!("Attampted to draw an unknown shape {}", shape)),
            }

            println!("{} created at index {}", shape, shape_idx);
        },
        [shape, _, _, ..] => return Err(format!("Too many arguments. Only X and Y arguments required to draw {}", shape)),
        [shape, ..] => return Err(format!("X and Y arguments required to draw {}", shape)),
        [] => return Err(String::from("Shape and coordinates are required to draw.")),
    }

    Ok(())
}


/// Handle Moving shapes
fn cmd_move(grid: &mut Grid, args: &[&str]) -> Result<(), String> {
    match args[..] {
        [i, x, y] => {
            let i = utils::str_to_usize(i)?;
            let x = utils::str_to_float(x)?;
            let y = utils::str_to_float(y)?;

            let shape = grid.get_shape_mut(i);

            if let Some(shape) = shape {
                shape.as_mut().move_to(x, y);
            } else {
                return Err(format!("No shape found at index {}", i));
            }
        },
        _ => return Err(String::from("The following values are required to move a shape: [shape_index, new_x, new_y]"))
    }

    Ok(())
}


/// Handle resizing shapes
fn cmd_resize(grid: &mut Grid, args: &[&str]) -> Result<(), String> {
    match args[..] {
        ["rect", i, w, h] => {
            let i = utils::str_to_usize(i)?;
            let w = utils::str_to_float(w)?;
            let h = utils::str_to_float(h)?;

            let shape = grid.get_shape_mut(i);

            if let Some(shape) = shape {
                shape.as_mut().resize(Dimensions::Double(w, h)).unwrap();
            } else {
                return Err(format!("No shape found at index {}", i));
            }
        },
        ["circle", i, r] => {
            let i = utils::str_to_usize(i)?;
            let r = utils::str_to_float(r)?;

            let shape = grid.get_shape_mut(i);

            if let Some(shape) = shape {
                shape.as_mut().resize(Dimensions::Single(r)).unwrap();
            } else {
                return Err(format!("No shape found at index {}", i));
            }
        },
        ["path", i, j, x, y] => {
            let i = utils::str_to_usize(i)?;
            let j = utils::str_to_usize(j)?;
            let x = utils::str_to_float(x)?;
            let y = utils::str_to_float(y)?;

            let shape = grid.get_shape_mut(i);

            if let Some(shape) = shape {
                shape.as_mut().resize(Dimensions::IndexPosition(j, x, y)).unwrap();
            } else {
                return Err(format!("No shape found at index {}", i));
            }
        },
        ["rect", ..] => return Err(String::from("Index, width, and height are required to resize a rect.")),
        ["circle", ..] => return Err(String::from("Index and radius are required to resize a circle.")),
        ["path", ..] => return Err(String::from("Shape index, point index, new_x, and new_y are required to resize a path.")),
        [shape, ..] => return Err(format!("Unable to resize unknown shape {}", shape)),
        _ => return Err(String::from("A shape type, index, and new dimensions are required for resizing.")),
    }

    Ok(())
}