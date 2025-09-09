// Game Of Life (gol)
mod gol {
    // Struct representing a Plane
    pub struct Plane {
        width: i32,
        length: i32,
        current: Vec<Vec<CellState>>,
        next: Vec<Vec<CellState>>,
    }

    pub enum CellState {
        Alive,
        Dead,
    }

    fn new_matrix(width: i32, length: i32) -> Vec<Vec<CellState>> {
        let mut matrix = Vec::new();

        for _ in 0..length {
            let mut row = Vec::new();

            for _ in 0..width {
                row.push(CellState::Dead);
            }

            matrix.push(row);
        }
        return matrix;
    }

    impl Plane {
        pub fn new(width: i32, length: i32) -> Self {
            Self {
                width,
                length,
                current: new_matrix(width, length),
                next: new_matrix(width, length),
            }
        }

        // fn trim_coords(x, y) -> (i32, i32)

        // fn count_alive_neighbors(x, y) -> u8

        // fn evolve(&self)

        pub fn print(&self) {
            for row in &self.current {
                for item in row {
                    match item {
                        CellState::Alive => print!("X"),
                        CellState::Dead => print!(" "),
                    }
                }
                println!();
            }
        }
    }
}

fn main() {
    let width = 10;
    let length = 10;

    let plane = gol::Plane::new(width, length);

    // plane.set_cell(x, y, alive or dead);

    let mut count = 0;
    loop {
        // clear screen
        plane.print();

        // plane.evolve()
        count += 1;
        if count == 10 {
            break;
        }
        // wait
    }
}
