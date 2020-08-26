use engine::Engine;

#[derive(Copy, Clone)]
pub struct Car {
    speed: u32,
    mpg_city: u32,
    mpg_highway: u32,
    weight: u32,
    make: &'static str,
    model: &'static str,
    year: u32,
    seats: u32,
    engine: Engine,
    gears: [u32; 7],
    msrp: f64,
}

impl Car {
    pub fn new(speed: u32) -> Car {
        Car {
            speed: speed,
            mpg_city: 12,
            mpg_highway: 18,
            weight: 4000,
            make: "Ford",
            model: "Mustang Shelby GT500",
            year: 2020,
            seats: 4,
            engine: Engine::new(),
            gears: [1, 2, 3, 4, 5, 6, 7],
            msrp: 72_900.00,
        }
    }

    pub fn accelerate(&mut self) {
        println!("current speed: {}", self.speed);
        self.speed += 1;
    }

    pub fn stats(&self) {
        println!(
            "current stats: {} {} {} {} {} {} {} {} {} {}",
            self.speed,
            self.mpg_city,
            self.mpg_highway,
            self.weight,
            self.make,
            self.model,
            self.year,
            self.seats,
            self.gears.len(),
            self.msrp
        );

        self.engine.stats();
    }
}
