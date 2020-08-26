#[derive(Copy, Clone)]
pub struct Engine {
    engine_type: &'static str,
    engine_displacement: f64,
    engine_torque: f64,
    engine_horsepower: f64,
    engine_array: [i32; 100],
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            engine_type: "5.2L Supercharged Cross Plane Crank V8",
            engine_displacement: 5.2,
            engine_torque: 625.0,
            engine_horsepower: 760.0,
            engine_array: [101; 100],
        }
    }

    pub fn stats(&self) {
        println!(
            "current stats: {} {} {} {} {}",
            self.engine_type,
            self.engine_displacement,
            self.engine_torque,
            self.engine_horsepower,
            self.engine_array.len(),
        );
    }
}
