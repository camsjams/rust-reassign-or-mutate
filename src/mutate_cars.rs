mod car;
mod engine;

const NUM_CARS: u32 = 100_000;
const TOTAL_EPOCHS: u64 = 5;

use car::Car;

fn main() {
    println!(
        "=== mutate_cars - {} v{} ====",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    );
    let mut cars: Vec<Car> = Vec::new();

    for i in 0..NUM_CARS {
        cars.push(Car::new(i));
        cars[i as usize].stats();
    }

    println!("Lets mutate!");

    for _i in 0..TOTAL_EPOCHS {
        for j in 0..cars.len() {
            &mut cars[j].accelerate();
        }
    }

    println!("Done!");
}
