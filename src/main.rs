use std::thread;
use std::time::Duration;

mod gol;

fn main() {
    let width = 50;
    let length = 30;

    let mut plane = gol::Plane::new(width, length);

    // try to set cell out of index
    // plane.set_cell(-1, 12, gol::CellState::Alive);

    // draw a glider
    plane.set_cell(4, 0, gol::CellState::Alive);
    plane.set_cell(5, 0, gol::CellState::Alive);
    plane.set_cell(6, 0, gol::CellState::Alive);
    plane.set_cell(6, 1, gol::CellState::Alive);
    plane.set_cell(5, 2, gol::CellState::Alive);

    loop {
        // clear screen
        print!("\x1B[2J\x1B[1;1H");

        plane.print();

        plane.evolve();

        // wait
        thread::sleep(Duration::from_millis(100));
    }
}
