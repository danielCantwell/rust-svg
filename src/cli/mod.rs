use std::io::prelude::*;
use std::io;

use crate::svg::{Grid, Point, SVG, Rect, Circle, Path};
use crate::utils;


///
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


///
pub fn execute_command(grid: &mut Grid, cmd: String) -> Result<(), String> {
    let args_iter : Vec<&str> = cmd.split_whitespace().collect();
    let root_arg = args_iter[0];
    let rest = &args_iter[1..];

    match root_arg {
        "html" => println!("{}", grid.to_html()),
        "draw" => cmd_draw(grid, rest)?,
        "move" => cmd_move(grid, rest)?,
        _ => {
            return Err(String::from(format!("Unable to parse command {}.", args_iter[0])));
        }
    }

    Ok(())
}


///
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


///
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