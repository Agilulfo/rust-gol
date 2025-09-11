use rust_gol as gol;

fn main() {
    let width = 50;
    let length = 30;

    let mut plane = gol::Plane::new_random(width, length);

    loop {
        // clear screen
        gol::utils::clear_terminal();

        plane.print();

        plane.evolve();

        // wait
        gol::utils::wait(100);
    }
}
