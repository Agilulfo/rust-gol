// Game Of Life (gol)
mod gol {
    #[derive(Debug)]
    pub struct Plane {
        width: i32,
        length: i32,
    }

    impl Plane {
        pub fn new(width: i32, length: i32) -> Self {
            Self { width, length }
        }

        pub fn area(&self) -> i32 {
            self.width * self.length
        }
    }
}

fn main() {
    let width = 10;
    let length = 10;

    let current_plane = gol::Plane::new(width, length);
    let next_plane = gol::Plane::new(width, length);

    dbg!(&current_plane);
    dbg!(&next_plane);

    println!("Current plane area is {}.", current_plane.area())
}
