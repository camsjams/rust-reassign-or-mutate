mod bicycle;

const NUM_BIKES: u32 = 100_000;
const TOTAL_EPOCHS: u64 = 5;

use bicycle::Bicycle;

fn main() {
    println!(
        "=== clone_bikes - {} v{} ====",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    );
    let mut bikes: Vec<Bicycle> = Vec::new();

    for i in 0..NUM_BIKES {
        bikes.push(Bicycle::new(i));
    }

    println!("Lets reassign!");

    for _i in 0..TOTAL_EPOCHS {
        for j in 0..bikes.len() {
            let mut bike = bikes[j].clone();
            bike.accelerate();
            bikes[j] = bike;
        }
    }

    println!("Done!");
}
