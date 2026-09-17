// Inspect how the preferred tolerance classes rank against a given class.
//
// The ranking, the deviations and the distances all come straight from
// list_preferred; the independent check of that metric lives in the tests.
//
// Usage: cargo run --example find_preferred -- <size_mm> <tolerance_class>
// e.g.:  cargo run --example find_preferred -- 30 K6

use iso_286::{find_preferred, limits, list_preferred};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <size_mm> <tolerance_class>", args[0]);
        std::process::exit(1);
    }
    let size: f64 = args[1].parse().expect("size must be a number");
    let tc = &args[2];

    let target = limits(size, tc).unwrap_or_else(|e| panic!("limits({size}, {tc}) failed: {e}"));
    println!(
        "target {tc} @ {size}mm -> upper={:.5} middle={:.5} lower={:.5}",
        target.upper, target.middle, target.lower
    );

    // Already closest first, so nothing here needs to sort or score.
    let ranked = list_preferred(size, tc)
        .unwrap_or_else(|e| panic!("list_preferred({size}, {tc}) failed: {e}"));

    println!(
        "\n{:<6} {:>10} {:>10} {:>10} {:>10}",
        "class", "upper", "middle", "lower", "error"
    );
    for m in &ranked {
        println!(
            "{:<6} {:>10.5} {:>10.5} {:>10.5} {:>10.5}",
            m.class, m.upper, m.middle, m.lower, m.error
        );
    }

    let chosen = find_preferred(size, tc).unwrap();
    println!(
        "\nfind_preferred chose: {} (error {:.5})",
        chosen.class, chosen.error
    );
}
