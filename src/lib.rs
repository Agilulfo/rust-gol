use rand;
use rand::Rng;
use std::mem;

pub mod utils;

pub struct Plane {
    width: i32,
    length: i32,
    // TODO: is there a way to define this as a type and reuse it where needed?
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

    pub fn new_random(width: i32, length: i32) -> Self {
        let mut plane = Self {
            width,
            length,
            current: new_matrix(width, length),
            next: new_matrix(width, length),
        };
        plane.randomize();
        return plane;
    }

    fn randomize(&mut self) {
        let mut rng = rand::rng();
        for x in 0..self.width {
            for y in 0..self.length {
                // TODO: how can I avoid having to cast to usize is it possible?
                // TODO: would be nice to generate directly one of the options, is this even possible?
                self.current[y as usize][x as usize] = if rng.random() {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
            }
        }
    }

    pub fn set_cell(&mut self, x: i32, y: i32, state: CellState) {
        let (x, y) = self.trim_coords(x, y);

        self.current[y][x] = state;
    }

    // TODO: can this be tested and remain private?
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

    // TODO: would be nice to implement a detection of a stall
    // we could keep a track of the recent n states (via hash)
    // and if an hash repeats we could somehow terminate the execution
    pub fn evolve(&mut self) {
        for x in 0..self.width {
            for y in 0..self.length {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                // TODO: maybe there is a way to write this that is easier to read
                // rules from wikipedia:
                //  - Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                //  - Any live cell with two or three live neighbours lives on to the next generation.
                //  - Any live cell with more than three live neighbours dies, as if by overpopulation.
                //  - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
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
