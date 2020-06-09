use rsvg::cli;
use rsvg::svg::{Grid, CoordinateSystem};


fn main() -> Result<(), String> {
    let mut grid = Grid::new(CoordinateSystem::MidMidUpRight);

    while let Ok(input) = cli::read_input() {
        if let Err(e) = cli::execute_command(&mut grid, input) {
            eprintln!("{}", e);
        }
    }

    Ok(())
}
