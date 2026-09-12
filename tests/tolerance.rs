use iso_286::{
    find_preferred, grades, hole_deviations, hole_preferred_tolerances, limits, list_preferred,
    shaft_deviations, shaft_preferred_tolerances,
};

// -- Spot checks across deviation groups --

#[test]
fn spot_check_holes() {
    // (tolerance_class, size_mm, expected_upper, expected_lower)
    let cases = [
        // A - G (positive deviations, hole)
        ("A7", 10.0, 0.295, 0.28),
        ("B7", 30.0, 0.181, 0.16),
        ("C7", 50.0, 0.155, 0.13),
        ("D7", 10.0, 0.055, 0.04),
        ("E7", 10.0, 0.04, 0.025),
        ("F7", 10.0, 0.028, 0.013),
        ("G7", 10.0, 0.02, 0.005),
        // H
        ("H7", 10.0, 0.015, 0.0),
        ("H7", 52.8, 0.03, 0.0),
        ("H8", 30.0, 0.033, 0.0),
        ("H11", 100.0, 0.22, 0.0),
        // JS
        ("JS7", 10.0, 0.0075, -0.0075),
        // J
        ("J7", 10.0, 0.008, -0.007),
        // K
        ("K7", 10.0, 0.005, -0.01),
        // M
        ("M7", 10.0, 0.0, -0.015),
        // N
        ("N7", 10.0, -0.004, -0.019),
        // P - ZC
        ("P7", 10.0, -0.009, -0.024),
        ("S7", 30.0, -0.027, -0.048),
        ("ZC7", 30.0, -0.21, -0.231),
    ];

    for (tc, size, exp_upper, exp_lower) in cases {
        let t = limits(size, tc).unwrap();
        assert_eq!(t.upper, exp_upper, "{tc} @ {size}: upper");
        assert_eq!(t.lower, exp_lower, "{tc} @ {size}: lower");
    }
}

#[test]
fn spot_check_shafts() {
    let cases = [
        // a - g (negative deviations, shaft)
        ("a7", 10.0, -0.28, -0.295),
        ("b7", 30.0, -0.16, -0.181),
        ("c7", 50.0, -0.13, -0.155),
        ("d7", 10.0, -0.04, -0.055),
        ("e7", 10.0, -0.025, -0.04),
        ("f7", 10.0, -0.013, -0.028),
        ("g6", 52.8, -0.01, -0.029),
        // h
        ("h7", 10.0, 0.0, -0.015),
        ("h6", 30.0, 0.0, -0.013),
        ("h11", 100.0, 0.0, -0.22),
        // js
        ("js4", 5.4, 0.002, -0.002),
        ("js7", 10.0, 0.0075, -0.0075),
        // k
        ("k7", 10.0, 0.016, 0.001),
        // m - zc
        ("m7", 10.0, 0.021, 0.006),
        ("p7", 10.0, 0.03, 0.015),
        ("s7", 30.0, 0.056, 0.035),
    ];

    for (tc, size, exp_upper, exp_lower) in cases {
        let t = limits(size, tc).unwrap();
        assert_eq!(t.upper, exp_upper, "{tc} @ {size}: upper");
        assert_eq!(t.lower, exp_lower, "{tc} @ {size}: lower");
    }
}

// -- Boundary sizes --

#[test]
fn bracket_boundaries_h7() {
    let boundary_sizes = [
        3.0, 6.0, 10.0, 18.0, 30.0, 50.0, 80.0, 120.0, 180.0, 250.0, 315.0, 400.0, 500.0,
    ];
    for size in boundary_sizes {
        let t = limits(size, "H7").unwrap();
        assert_eq!(t.lower, 0.0, "H7 @ {size}: lower should be 0");
        assert!(t.upper > 0.0, "H7 @ {size}: upper should be positive");
    }
}

#[test]
fn just_over_bracket_boundary() {
    // 10.001 should land in the same bracket as 11 (10-18 bracket)
    let t1 = limits(10.001, "H7").unwrap();
    let t2 = limits(11.0, "H7").unwrap();
    assert_eq!(t1.upper, t2.upper);
    assert_eq!(t1.lower, t2.lower);
}

#[test]
fn just_under_bracket_boundary() {
    // 9.999 should land in the 6-10 bracket (ceil = 10)
    let t1 = limits(9.999, "H7").unwrap();
    let t2 = limits(10.0, "H7").unwrap();
    assert_eq!(t1.upper, t2.upper);
    assert_eq!(t1.lower, t2.lower);
}

// -- Invariants --

#[test]
fn ordering_invariant() {
    let sizes = [
        3.0, 6.0, 10.0, 18.0, 30.0, 50.0, 80.0, 120.0, 180.0, 250.0, 315.0, 400.0, 500.0,
    ];
    let test_classes = [
        "H7", "H8", "H11", "h7", "h6", "g6", "f7", "e8", "js7", "JS7", "k7", "K7", "m6", "M7",
        "n6", "N7", "p6", "P7", "s7", "S7",
    ];

    for size in sizes {
        for tc in test_classes {
            if let Ok(t) = limits(size, tc) {
                assert!(
                    t.upper >= t.middle,
                    "{tc} @ {size}: upper ({}) >= middle ({})",
                    t.upper,
                    t.middle
                );
                assert!(
                    t.middle >= t.lower,
                    "{tc} @ {size}: middle ({}) >= lower ({})",
                    t.middle,
                    t.lower
                );
            }
        }
    }
}

#[test]
fn js_symmetry() {
    let sizes = [3.0, 10.0, 30.0, 50.0, 120.0, 250.0, 500.0];
    let js_grades = [
        "JS4", "JS5", "JS6", "JS7", "JS8", "js4", "js5", "js6", "js7", "js8",
    ];

    for size in sizes {
        for tc in js_grades {
            if let Ok(t) = limits(size, tc) {
                assert_eq!(
                    t.upper, -t.lower,
                    "{tc} @ {size}: JS upper ({}) should equal -lower ({})",
                    t.upper, t.lower
                );
                assert_eq!(t.middle, 0.0, "{tc} @ {size}: JS middle should be 0");
            }
        }
    }
}

#[test]
fn h_lower_always_zero() {
    let sizes = [3.0, 10.0, 30.0, 80.0, 180.0, 500.0];
    for g in grades() {
        let tc = format!("H{g}");
        for &size in &sizes {
            if let Ok(t) = limits(size, &tc) {
                assert_eq!(t.lower, 0.0, "{tc} @ {size}: H lower should be 0");
            }
        }
    }
}

#[test]
fn h_shaft_upper_always_zero() {
    let sizes = [3.0, 10.0, 30.0, 80.0, 180.0, 500.0];
    for g in grades() {
        let tc = format!("h{g}");
        for &size in &sizes {
            if let Ok(t) = limits(size, &tc) {
                assert_eq!(t.upper, 0.0, "{tc} @ {size}: h upper should be 0");
            }
        }
    }
}

#[test]
fn middle_is_average() {
    let cases = [
        ("H7", 10.0),
        ("g6", 52.8),
        ("js4", 5.4),
        ("K7", 30.0),
        ("M7", 50.0),
        ("N7", 80.0),
        ("P7", 120.0),
        ("s7", 250.0),
    ];

    for (tc, size) in cases {
        let t = limits(size, tc).unwrap();
        let expected = (t.upper + t.lower) / 2.0;
        assert!(
            (t.middle - expected).abs() < 1e-9,
            "{tc} @ {size}: middle ({}) should be avg of upper ({}) and lower ({})",
            t.middle,
            t.upper,
            t.lower
        );
    }
}

// -- Edge cases and errors --

#[test]
fn invalid_tolerance_class() {
    assert!(limits(10.0, "X99").is_err());
    assert!(limits(10.0, "").is_err());
    assert!(limits(10.0, "777").is_err());
    assert!(limits(10.0, "H").is_err());
    assert!(limits(10.0, "7").is_err());
    assert!(limits(10.0, "HH7").is_err());
}

#[test]
fn size_out_of_range() {
    assert!(limits(4000.0, "H7").is_err());
    assert!(limits(10000.0, "g6").is_err());
}

#[test]
fn non_positive_or_non_finite_size_rejected() {
    for size in [0.0, -5.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            limits(size, "H7").is_err(),
            "size {size} should be rejected"
        );
        assert!(
            find_preferred(size, "H7").is_err(),
            "size {size} should be rejected by find_preferred"
        );
        assert!(
            list_preferred(size, "H7").is_err(),
            "size {size} should be rejected by list_preferred"
        );
    }
}

#[test]
fn unsupported_combinations_above_500() {
    // IT01 and IT0 are not available above 500mm
    assert!(limits(600.0, "H01").is_err());
    assert!(limits(600.0, "H0").is_err());
}

// -- Metadata functions --

#[test]
fn grades_complete() {
    let g = grades();
    assert_eq!(g.len(), 20);
    assert_eq!(g[0], "01");
    assert_eq!(g[1], "0");
    assert_eq!(g[19], "18");
}

#[test]
fn hole_deviations_uppercase() {
    let h = hole_deviations();
    assert_eq!(h.len(), 28);
    for d in &h {
        assert_eq!(
            *d,
            d.to_uppercase(),
            "hole deviation {d} should be uppercase"
        );
    }
    assert_eq!(h[0], "A");
    assert_eq!(h[27], "ZC");
}

#[test]
fn shaft_deviations_lowercase() {
    let s = shaft_deviations();
    assert_eq!(s.len(), 28);
    for d in &s {
        assert_eq!(
            *d,
            d.to_lowercase(),
            "shaft deviation {d} should be lowercase"
        );
    }
    assert_eq!(s[0], "a");
    assert_eq!(s[27], "zc");
}

// -- Cross-group coverage: every deviation letter produces a result for at least one common case --

#[test]
fn all_hole_deviations_resolve() {
    let h = hole_deviations();
    for d in &h {
        let tc = format!("{d}7");
        let result = limits(30.0, &tc);
        assert!(
            result.is_ok(),
            "hole deviation {tc} @ 30mm should resolve, got {:?}",
            result.err()
        );
    }
}

#[test]
fn all_shaft_deviations_resolve() {
    let s = shaft_deviations();
    for d in s {
        let tc = format!("{d}7");
        let result = limits(50.0, &tc);
        assert!(
            result.is_ok(),
            "shaft deviation {tc} @ 50mm should resolve, got {:?}",
            result.err()
        );
    }
}

// -- list_preferred / find_preferred --

// Recompute the same "sum of absolute deviation differences" error that
// list_preferred/find_preferred use internally, so tests can check ordering
// without depending on their private implementation.
fn preferred_error(size: f64, tolerance_class: &str, preferred_class: &str) -> f64 {
    let target = limits(size, tolerance_class).unwrap();
    let candidate = limits(size, preferred_class).unwrap();
    (target.upper - candidate.upper).abs()
        + (target.middle - candidate.middle).abs()
        + (target.lower - candidate.lower).abs()
}

#[test]
fn find_preferred_returns_head_of_list_preferred() {
    let cases = [
        ("H6", 10.0),
        ("G6", 30.0),
        ("K6", 30.0),
        ("M6", 80.0),
        ("h5", 30.0),
        ("f6", 30.0),
        ("n7", 80.0),
        ("M7", 638.0),
    ];
    for (tc, size) in cases {
        let list = list_preferred(size, tc).unwrap();
        let found = find_preferred(size, tc).unwrap();
        assert_eq!(
            found, list[0],
            "find_preferred({tc}, {size}) should equal list_preferred's first entry"
        );
    }
}

#[test]
fn list_preferred_is_sorted_ascending_by_error() {
    let cases = [
        ("H6", 10.0),
        ("K6", 30.0),
        ("M6", 80.0),
        ("n7", 80.0),
        ("M7", 638.0),
    ];
    for (tc, size) in cases {
        let list = list_preferred(size, tc).unwrap();
        let errors: Vec<f64> = list
            .iter()
            .map(|pc| preferred_error(size, tc, pc))
            .collect();
        for w in errors.windows(2) {
            assert!(
                w[0] <= w[1],
                "{tc} @ {size}: list_preferred not sorted ascending: {list:?} -> {errors:?}"
            );
        }
    }
}

#[test]
fn list_preferred_only_contains_valid_candidates_for_size() {
    // At 638mm several small-size-only preferred classes (e.g. A11) become
    // unsupported; list_preferred must omit them rather than propagate an
    // error or include a class that limits() itself rejects.
    let list = list_preferred(638.0, "M7").unwrap();
    assert!(
        !list.contains(&"A11".to_string()),
        "A11 is unsupported at 638mm and should be excluded, got {list:?}"
    );
    for pc in &list {
        assert!(
            limits(638.0, pc).is_ok(),
            "{pc} in list_preferred(638.0, \"M7\") should itself resolve via limits()"
        );
    }
}

#[test]
fn preferred_class_maps_to_itself() {
    // Every preferred class, evaluated at a size where it's valid, should
    // be its own closest match (error 0, so it should sort first).
    let sizes = [3.0, 10.0, 30.0, 80.0, 180.0, 400.0];
    for tc in hole_preferred_tolerances() {
        for &size in &sizes {
            if limits(size, &tc).is_ok() {
                assert_eq!(
                    find_preferred(size, &tc).unwrap(),
                    tc,
                    "hole {tc} @ {size} should map to itself"
                );
                assert_eq!(
                    list_preferred(size, &tc).unwrap()[0],
                    tc,
                    "hole {tc} @ {size} should be its own first choice"
                );
            }
        }
    }
    for tc in shaft_preferred_tolerances() {
        for &size in &sizes {
            if limits(size, &tc).is_ok() {
                assert_eq!(
                    find_preferred(size, &tc).unwrap(),
                    tc,
                    "shaft {tc} @ {size} should map to itself"
                );
                assert_eq!(
                    list_preferred(size, &tc).unwrap()[0],
                    tc,
                    "shaft {tc} @ {size} should be its own first choice"
                );
            }
        }
    }
}

#[test]
fn hole_class_maps_to_hole_preferred() {
    let hole_preferred = hole_preferred_tolerances();
    for tc in ["H6", "G6", "K6", "M6", "R6", "ZC7"] {
        let result = find_preferred(30.0, tc).unwrap();
        assert!(
            hole_preferred.contains(&result),
            "{tc} should map to a preferred hole class, got {result}"
        );

        let list = list_preferred(30.0, tc).unwrap();
        assert!(
            list.iter().all(|pc| hole_preferred.contains(pc)),
            "{tc}: list_preferred should only contain preferred hole classes, got {list:?}"
        );
    }
}

#[test]
fn shaft_class_maps_to_shaft_preferred() {
    let shaft_preferred = shaft_preferred_tolerances();
    for tc in ["h5", "f6", "m5", "n7", "u6"] {
        let result = find_preferred(30.0, tc).unwrap();
        assert!(
            shaft_preferred.contains(&result),
            "{tc} should map to a preferred shaft class, got {result}"
        );

        let list = list_preferred(30.0, tc).unwrap();
        assert!(
            list.iter().all(|pc| shaft_preferred.contains(pc)),
            "{tc}: list_preferred should only contain preferred shaft classes, got {list:?}"
        );
    }
}

#[test]
fn find_preferred_invalid_tolerance_class() {
    assert!(find_preferred(10.0, "X99").is_err());
    assert!(find_preferred(10.0, "").is_err());
}

#[test]
fn list_preferred_invalid_tolerance_class() {
    assert!(list_preferred(10.0, "X99").is_err());
    assert!(list_preferred(10.0, "").is_err());
}

#[test]
fn find_preferred_size_out_of_range() {
    assert!(find_preferred(4000.0, "H7").is_err());
}

#[test]
fn list_preferred_size_out_of_range() {
    assert!(list_preferred(4000.0, "H7").is_err());
}

#[test]
fn find_preferred_skips_unsupported_candidates() {
    // At 638mm, the "A11" preferred candidate is unsupported (out of range
    // for that grade), but find_preferred should still return the best
    // among the candidates that *are* valid, not bail out entirely.
    assert_eq!(find_preferred(638.0, "M7").unwrap(), "N7");
}

#[test]
fn find_preferred_close_call_h6_vs_h7() {
    // H6 at 10mm (0/+0.009) is closer to preferred H7 (0/+0.015) than to
    // any other preferred hole class.
    assert_eq!(find_preferred(10.0, "H6").unwrap(), "H7");
}
