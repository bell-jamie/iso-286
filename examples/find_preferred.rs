// Manual sanity-check for the find_preferred closest-match metric.
//
// Usage: cargo run --example find_preferred -- <size_mm> <tolerance_class>
// e.g.:  cargo run --example find_preferred -- 30 K6

use iso_286::{find_preferred, hole_preferred_tolerances, limits, shaft_preferred_tolerances};

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

    let hole = tc.chars().next().unwrap().is_uppercase();
    let preferred = if hole {
        hole_preferred_tolerances()
    } else {
        shaft_preferred_tolerances()
    };

    let mut rows: Vec<(String, f64, f64, f64, f64)> = preferred
        .iter()
        .filter_map(|pc| {
            let t = limits(size, pc).ok()?;
            let error =
                (target.upper - t.upper).abs() + (target.middle - t.middle).abs() + (target.lower - t.lower).abs();
            Some((pc.clone(), t.upper, t.middle, t.lower, error))
        })
        .collect();
    rows.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap());

    println!("\n{:<6} {:>10} {:>10} {:>10} {:>10}", "class", "upper", "middle", "lower", "error");
    for (pc, upper, middle, lower, error) in &rows {
        println!("{pc:<6} {upper:>10.5} {middle:>10.5} {lower:>10.5} {error:>10.5}");
    }

    let chosen = find_preferred(size, tc).unwrap();
    println!("\nfind_preferred chose: {chosen}");
}
