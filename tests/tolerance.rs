use iso_286::{grades, hole_deviations, limits, shaft_deviations};

#[test]
fn test_limits() {
    let cases = [
        ("H7", 10.0, 0.015, 0.000),
        ("js4", 5.4, 0.002, -0.002),
        ("H7", 52.8, 0.030, 0.000),
        ("g6", 52.8, -0.010, -0.029),
    ];

    for (tolerance_class, size, exp_upper, exp_lower) in cases {
        let t = limits(size, tolerance_class).unwrap();
        assert_eq!(t.upper, exp_upper, "{tolerance_class} @ {size}: upper");
        assert_eq!(t.lower, exp_lower, "{tolerance_class} @ {size}: lower");
    }
}

#[test]
fn test_tolerance_class_variants() {
    assert_eq!(limits(10.0, "H7").unwrap().upper, 0.015);
    assert_eq!(limits(5.4, "js4").unwrap().lower, -0.002);
    assert!(limits(52.8, "ZC5").is_ok());
}

#[test]
fn test_invalid_tolerance_class() {
    assert!(limits(10.0, "X99").is_err());
    assert!(limits(10.0, "").is_err());
    assert!(limits(10.0, "777").is_err());
}

#[test]
fn test_grades() {
    let g = grades();
    assert_eq!(g[0], "01");
    assert_eq!(g[1], "0");
    assert!(g.len() == 20);
}

#[test]
fn test_hole_deviations() {
    let h = hole_deviations();
    assert_eq!(h[0], "A");
    assert_eq!(h[11], "JS");
    assert!(h.len() == 28);
}

#[test]
fn test_shaft_deviations() {
    let s = shaft_deviations();
    assert_eq!(s[0], "a");
    assert_eq!(s[11], "js");
    assert!(s.len() == 28);
}
