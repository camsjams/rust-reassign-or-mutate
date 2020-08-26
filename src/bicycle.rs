#[derive(Copy, Clone)]
pub struct Bicycle {
    speed: u32,
}

impl Bicycle {
    pub fn new(speed: u32) -> Bicycle {
        Bicycle { speed: speed }
    }

    pub fn accelerate(&mut self) {
        println!("current speed: {}", self.speed);
        self.speed += 1;
    }
}
