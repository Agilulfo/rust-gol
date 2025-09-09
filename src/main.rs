// Game Of Life (gol)
mod gol {
    use std::mem;
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

        pub fn set_cell(&mut self, x: i32, y: i32, state: CellState) {
            let (x, y) = self.trim_coords(x, y);

            self.current[y][x] = state;
        }

        fn trim_coords(&self, x: i32, y: i32) -> (usize, usize) {
            let x = if x >= self.width {
                x % self.width
            } else if x < 0 {
                self.width - (-x % self.width)
            } else {
                x
            };

            let y = if y >= self.length {
                y % self.length
            } else if y < 0 {
                self.length - (-y % self.length)
            } else {
                y
            };

            (x as usize, y as usize)
        }

        fn count_alive_neighbors(&self, x: i32, y: i32) -> u8 {
            let neighbors = [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ];

            let mut count = 0;
            for (x, y) in neighbors {
                let (x, y) = self.trim_coords(x, y);

                if let CellState::Alive = self.current[y][x] {
                    count += 1
                };
            }
            return count;
        }

        pub fn evolve(&mut self) {
            for x in 0..self.width {
                for y in 0..self.length {
                    let alive_neighbors = self.count_alive_neighbors(x, y);

                    let next_state = match self.current[y as usize][x as usize] {
                        CellState::Alive => {
                            if alive_neighbors == 2 || alive_neighbors == 3 {
                                CellState::Alive
                            } else {
                                CellState::Dead
                            }
                        }
                        CellState::Dead => {
                            if alive_neighbors == 3 {
                                CellState::Alive
                            } else {
                                CellState::Dead
                            }
                        }
                    };

                    self.next[y as usize][x as usize] = next_state;
                }
            }

            mem::swap(&mut self.current, &mut self.next);
        }

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

    let mut plane = gol::Plane::new(width, length);

    // try to set cell out of index
    // plane.set_cell(-1, 12, gol::CellState::Alive);

    // draw a glider
    plane.set_cell(4, 0, gol::CellState::Alive);
    plane.set_cell(5, 0, gol::CellState::Alive);
    plane.set_cell(6, 0, gol::CellState::Alive);
    plane.set_cell(6, 1, gol::CellState::Alive);
    plane.set_cell(5, 2, gol::CellState::Alive);

    let mut count = 0;
    loop {
        // clear screen
        plane.print();

        plane.evolve();
        count += 1;
        if count == 10 {
            break;
        }
        // wait
    }
}
