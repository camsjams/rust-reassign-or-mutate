# Rust: Reassign (Clone or Copy) vs mutate?
A comparison between reassignment and mutation in Rust

![Image of rusty stairs into water](img/pexels-mike-282005.jpg)

## Version
See [Cargo.toml](Cargo.toml) version

## Platforms / Technologies
* [Rust](https://www.rust-lang.org/en-US/)
* [Cargo](https://doc.rust-lang.org/cargo/)

## Description
See this companion [Medium post]() for more information.

## Build
>      $ cargo build

## Run quick version (pick one)
>      $ cargo run --bin clone_bikes
>      $ cargo run --bin mutate_bikes
>      $ cargo run --bin clone_cars
>      $ cargo run --bin copy_cars
>      $ cargo run --bin mutate_cars

## Build For Release
>      $ cargo build --release

## Run release version (pick one)
>      $ target/release/clone_bikes
>      $ target/release/mutate_bikes
>      $ target/release/clone_cars
>      $ target/release/copy_cars
>      $ target/release/mutate_cars

## Run with heaptrack
>      $ heaptrack <your desired bin>

### Example
>      $ heaptrack target/release/mutate_bikes
