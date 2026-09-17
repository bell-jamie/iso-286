// Inspect the closest tolerance classes to a wanted size and tolerance, for a hole and
// a shaft, with the nominal size held and free.
//
// Usage: cargo run --example closest -- <size_mm> <upper_mm> <lower_mm> [results] [decimals]
// e.g.:  cargo run --example closest -- 10.37 0.021 0.003
//        cargo run --example closest -- 52.8 0.03 -0.012 8 1

use iso_286::{Feature, list_closest};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !(4..=6).contains(&args.len()) {
        eprintln!(
            "usage: {} <size_mm> <upper_mm> <lower_mm> [results] [decimals]",
            args[0]
        );
        std::process::exit(1);
    }
    let size: f64 = args[1].parse().expect("size must be a number");
    let upper: f64 = args[2].parse().expect("upper must be a number");
    let lower: f64 = args[3].parse().expect("lower must be a number");
    let results: usize = args
        .get(4)
        .map_or(Ok(5), |arg| arg.parse())
        .expect("results must be a whole number");
    // Default to the precision the size was given at, so the free search is always
    // allowed to name the supplied size and the two modes start out comparable.
    let decimals: usize = args
        .get(5)
        .map_or(Ok(decimals_in(&args[1])), |arg| arg.parse())
        .expect("decimals must be a whole number");

    println!(
        "target {:.4} / {:.4}  ({results} results, {decimals} dp)",
        size + upper,
        size + lower
    );

    for feature in [Feature::Hole, Feature::Shaft] {
        for strict in [true, false] {
            println!("\n{size} {upper:+}/{lower:+}  {feature:?}, strict = {strict}");
            match list_closest(size, upper, lower, feature, strict, results, decimals) {
                Ok(found) => {
                    for m in found {
                        println!(
                            "  {:>9} {:<5}  {:.4} / {:.4}   error {:.5}",
                            m.size,
                            m.class,
                            m.size + m.upper,
                            m.size + m.lower,
                            m.error
                        );
                    }
                }
                Err(e) => println!("  {e}"),
            }
        }
    }
}

// How many decimal places a size was written to, e.g. "10.37" -> 2.
fn decimals_in(size: &str) -> usize {
    size.split_once('.')
        .map_or(0, |(_, fraction)| fraction.len())
        .min(6)
}
