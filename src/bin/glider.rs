use rust_gol as gol;

fn main() {
    let width = 50;
    let length = 30;

    let mut plane = gol::Plane::new(width, length);

    // draw a glider
    plane.set_cell(4, 0, gol::CellState::Alive);
    plane.set_cell(5, 0, gol::CellState::Alive);
    plane.set_cell(6, 0, gol::CellState::Alive);
    plane.set_cell(6, 1, gol::CellState::Alive);
    plane.set_cell(5, 2, gol::CellState::Alive);

    loop {
        // clear screen
        gol::utils::clear_terminal();

        plane.print();

        plane.evolve();

        // wait
        gol::utils::wait(100);
    }
}
