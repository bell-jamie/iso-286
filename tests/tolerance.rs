use iso_286::{
    Error, Feature, Match, find_preferred, grades, hole_deviations, hole_preferred_tolerances,
    limits, list_closest, list_preferred, shaft_deviations, shaft_preferred_tolerances,
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
        + (target.lower - candidate.lower).abs()
        + ((target.upper + target.lower) / 2.0 - (candidate.upper + candidate.lower) / 2.0).abs()
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
        assert_eq!(found.size, size);
        assert_eq!(found.tolerance(), limits(size, &found.class).unwrap());
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
            .map(|m| preferred_error(size, tc, &m.class))
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
        !list.iter().any(|m| m.class == "A11"),
        "A11 is unsupported at 638mm and should be excluded, got {list:?}"
    );
    for m in &list {
        assert!(
            limits(638.0, &m.class).is_ok(),
            "{} in list_preferred(638.0, \"M7\") should itself resolve via limits()",
            m.class
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
                    find_preferred(size, &tc).unwrap().class,
                    tc,
                    "hole {tc} @ {size} should map to itself"
                );
                assert_eq!(
                    list_preferred(size, &tc).unwrap()[0].class,
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
                    find_preferred(size, &tc).unwrap().class,
                    tc,
                    "shaft {tc} @ {size} should map to itself"
                );
                assert_eq!(
                    list_preferred(size, &tc).unwrap()[0].class,
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
            hole_preferred.contains(&result.class),
            "{tc} should map to a preferred hole class, got {}",
            result.class
        );

        let list = list_preferred(30.0, tc).unwrap();
        assert!(
            list.iter().all(|m| hole_preferred.contains(&m.class)),
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
            shaft_preferred.contains(&result.class),
            "{tc} should map to a preferred shaft class, got {}",
            result.class
        );

        let list = list_preferred(30.0, tc).unwrap();
        assert!(
            list.iter().all(|m| shaft_preferred.contains(&m.class)),
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
    assert_eq!(find_preferred(638.0, "M7").unwrap().class, "N7");
}

#[test]
fn find_preferred_close_call_h6_vs_h7() {
    // H6 at 10mm (0/+0.009) is closer to preferred H7 (0/+0.015) than to
    // any other preferred hole class.
    assert_eq!(find_preferred(10.0, "H6").unwrap().class, "H7");
}

// -- list_closest, strict --

// Mirrors the metric list_closest ranks on, so tests can check the ordering from outside.
fn error(size: f64, class: &str, upper: f64, lower: f64) -> f64 {
    let t = limits(size, class).unwrap();
    (t.upper - upper).abs()
        + (t.lower - lower).abs()
        + ((t.upper + t.lower) / 2.0 - (upper + lower) / 2.0).abs()
}

#[test]
fn strict_finds_an_exact_match() {
    // H7 at 10 mm is exactly +0.015 / 0, so it has to come first.
    let found = list_closest(10.0, 0.015, 0.0, Feature::Hole, true, 3, 1).unwrap();
    assert_eq!(found[0].size, 10.0);
    assert_eq!(found[0].class, "H7");
    assert_eq!(found[0].error, 0.0);
    assert_eq!(found[0].tolerance(), limits(10.0, "H7").unwrap());
    assert_eq!(found.len(), 3);
}

#[test]
fn strict_leaves_the_nominal_size_alone() {
    let size = 10.4;
    for found in list_closest(size, 0.02, -0.01, Feature::Hole, true, 10, 1).unwrap() {
        assert_eq!(found.size, size);
    }
}

#[test]
fn strict_is_ordered_by_ascending_error() {
    let (size, upper, lower) = (52.8, 0.03, -0.012);
    let found = list_closest(size, upper, lower, Feature::Hole, true, 20, 1).unwrap();
    let errors: Vec<f64> = found
        .iter()
        .map(|m| error(size, &m.class, upper, lower))
        .collect();
    // Recomputing in millimetres reintroduces the float noise the library avoids by
    // ranking in integer nanometres, so allow a slack of well under one nanometre.
    assert!(errors.windows(2).all(|w| w[0] <= w[1] + 1e-9), "{errors:?}");
}

#[test]
fn strict_returns_the_global_best() {
    // The head of a one result search must beat every class the tables can produce.
    let (size, upper, lower) = (85.0, 0.041, 0.002);
    let found = list_closest(size, upper, lower, Feature::Hole, true, 1, 1).unwrap();
    let best = error(size, &found[0].class, upper, lower);

    for deviation in hole_deviations() {
        for grade in grades() {
            let class = format!("{deviation}{grade}");
            if limits(size, &class).is_ok() {
                assert!(
                    error(size, &class, upper, lower) >= best,
                    "{class} beats {}",
                    found[0].class
                );
            }
        }
    }
}

#[test]
fn tolerance_limits_may_be_supplied_either_way_round() {
    let ordered = list_closest(10.0, 0.015, 0.0, Feature::Hole, true, 5, 1).unwrap();
    let swapped = list_closest(10.0, 0.0, 0.015, Feature::Hole, true, 5, 1).unwrap();
    assert_eq!(ordered, swapped);
}

#[test]
fn results_are_capped_not_padded() {
    assert!(
        list_closest(10.0, 0.015, 0.0, Feature::Hole, true, 0, 1)
            .unwrap()
            .is_empty()
    );
    // Far more than the tables can supply for one size.
    let all = list_closest(1.0, 0.01, 0.0, Feature::Hole, true, 5_000, 1).unwrap();
    assert!(!all.is_empty() && all.len() < 5_000);
}

#[test]
fn list_closest_rejects_bad_arguments() {
    assert!(matches!(
        list_closest(0.0, 0.015, 0.0, Feature::Hole, true, 3, 1),
        Err(Error::SizeOutOfRange(_))
    ));
    assert!(matches!(
        list_closest(4000.0, 0.015, 0.0, Feature::Hole, true, 3, 1),
        Err(Error::SizeOutOfRange(_))
    ));
    assert!(matches!(
        list_closest(10.0, f64::NAN, 0.0, Feature::Hole, true, 3, 1),
        Err(Error::InvalidTolerance { .. })
    ));
    assert!(matches!(
        list_closest(10.0, 0.015, 0.0, Feature::Hole, true, 3, 7),
        Err(Error::InvalidPrecision(7))
    ));
}

// -- list_closest, free nominal size --

#[test]
fn free_search_may_move_the_nominal_size_onto_an_exact_match() {
    // 10 h7 is 10.000/9.985. Asking for those two limits as a hole, every IT7 class
    // whose lower deviation keeps the nominal inside the same bracket can hit them
    // exactly, but only by shifting the nominal size off 10 mm.
    let target = (10.0, 9.985);
    let found = list_closest(10.0, 0.0, -0.015, Feature::Hole, false, 4, 3).unwrap();

    for m in &found {
        assert!(
            absolute_error(m.size, &m.class, target) < 1e-9,
            "{} {} is not an exact fit, in {found:?}",
            m.size,
            m.class
        );
    }
    assert!(
        found.iter().any(|m| m.size == 9.985 && m.class == "H7"),
        "expected 9.985 H7 among {found:?}"
    );
    assert!(
        found.iter().any(|m| m.size != 10.0),
        "nothing moved off the supplied size in {found:?}"
    );
}

#[test]
fn free_search_holds_the_wanted_precision() {
    for decimals in 0..=3 {
        let step = 10f64.powi(decimals as i32);
        for m in list_closest(10.37, 0.02, -0.01, Feature::Shaft, false, 25, decimals).unwrap() {
            assert!(
                ((m.size * step).round() / step - m.size).abs() < 1e-9,
                "{} {} is finer than {decimals} decimals",
                m.size,
                m.class
            );
        }
    }
}

// The same metric again, but against the absolute limits of the feature, which is what
// the free search compares since a candidate may sit on a different nominal size.
fn absolute_error(found: f64, class: &str, target: (f64, f64)) -> f64 {
    let t = limits(found, class).unwrap();
    let (upper, lower) = (found + t.upper, found + t.lower);
    (upper - target.0).abs()
        + (lower - target.1).abs()
        + ((upper + lower) / 2.0 - (target.0 + target.1) / 2.0).abs()
}

#[test]
fn free_search_is_never_worse_than_strict() {
    // Only where the supplied size is already on the grid `decimals` asks for. Strict
    // holds the size whatever its precision, so otherwise it can reach a nominal size
    // the free search is not allowed to name.
    let cases = [
        (10.0, 0.015, 0.0, 1),
        (10.37, 0.02, -0.01, 2),
        (52.8, 0.03, -0.012, 1),
        (0.6, 0.004, -0.004, 1),
        (2750.0, 0.4, 0.1, 1),
    ];
    for (size, upper, lower, decimals) in cases {
        let target = (size + upper, size + lower);
        let strict = list_closest(size, upper, lower, Feature::Hole, true, 1, decimals).unwrap();
        let free = list_closest(size, upper, lower, Feature::Hole, false, 1, decimals).unwrap();

        let strict_error = absolute_error(strict[0].size, &strict[0].class, target);
        let free_error = absolute_error(free[0].size, &free[0].class, target);
        assert!(
            free_error <= strict_error + 1e-9,
            "{size} {upper}/{lower}: free {free:?} ({free_error}) lost to strict \
             {strict:?} ({strict_error})"
        );
    }
}

#[test]
fn every_result_is_a_class_that_applies_at_the_size_returned() {
    // Also pins the bracket table: a candidate is scored at its bracket's upper bound,
    // so a nominal size elsewhere in that bracket has to resolve the same way.
    let cases = [
        (10.0, 0.015, 0.0, 3),
        (10.37, 0.02, -0.01, 1),
        (0.6, 0.004, -0.004, 2),
        (137.5, 0.05, -0.05, 0),
        (2750.0, 0.4, 0.1, 1),
    ];
    for (size, upper, lower, decimals) in cases {
        for (feature, strict) in [
            (Feature::Hole, true),
            (Feature::Hole, false),
            (Feature::Shaft, true),
            (Feature::Shaft, false),
        ] {
            for m in list_closest(size, upper, lower, feature, strict, 200, decimals).unwrap() {
                assert!(m.size > 0.0 && m.size <= 3150.0, "{} out of range", m.size);
                assert!(
                    limits(m.size, &m.class).is_ok(),
                    "{} does not apply at {}",
                    m.class,
                    m.size
                );
            }
        }
    }
}

// -- list_closest, width mismatch bound --

#[test]
fn the_bound_does_not_change_the_ranking() {
    // A shorter list prunes harder, so it has to stay a prefix of a longer one.
    for (feature, strict) in [
        (Feature::Hole, true),
        (Feature::Hole, false),
        (Feature::Shaft, true),
        (Feature::Shaft, false),
    ] {
        let long = list_closest(10.37, 0.021, 0.003, feature, strict, 40, 2).unwrap();
        for n in 1..=40 {
            let short = list_closest(10.37, 0.021, 0.003, feature, strict, n, 2).unwrap();
            assert_eq!(
                short.as_slice(),
                &long[..n.min(long.len())],
                "{feature:?} strict = {strict}, results = {n}"
            );
        }
    }
}

#[test]
fn free_search_beats_every_class_near_the_size() {
    // Exhaustive over a 4 mm window of the size grid, which is far wider than any
    // worthwhile answer: moving the nominal a whole millimetre costs a thousand times
    // the tolerance being matched.
    let (size, upper, lower, decimals) = (10.37, 0.021, 0.003, 2);
    let target = (size + upper, size + lower);
    let found = list_closest(size, upper, lower, Feature::Hole, false, 1, decimals).unwrap();
    let best = absolute_error(found[0].size, &found[0].class, target);

    let step = 10f64.powi(decimals as i32);
    for offset in -200..=200 {
        let nominal = ((size * step).round() + f64::from(offset)) / step;
        for deviation in hole_deviations() {
            for grade in grades() {
                let class = format!("{deviation}{grade}");
                if limits(nominal, &class).is_ok() {
                    assert!(
                        absolute_error(nominal, &class, target) >= best - 1e-9,
                        "{nominal} {class} beats {found:?}"
                    );
                }
            }
        }
    }
}

// -- list_closest, the reported tolerance and error --

#[test]
fn the_reported_tolerance_is_the_one_the_class_gives() {
    let cases = [
        (10.0, 0.015, 0.0, 1, true),
        (10.37, 0.021, 0.003, 2, false),
        (137.5, 0.05, -0.05, 0, false),
        (0.6, 0.004, -0.004, 3, false),
    ];
    for (size, upper, lower, decimals, strict) in cases {
        for feature in [Feature::Hole, Feature::Shaft] {
            for m in list_closest(size, upper, lower, feature, strict, 20, decimals).unwrap() {
                assert_eq!(
                    m.tolerance(),
                    limits(m.size, &m.class).unwrap(),
                    "{} {} reported the wrong deviations",
                    m.size,
                    m.class
                );
            }
        }
    }
}

#[test]
fn the_reported_error_is_the_distance_from_the_wanted_tolerance() {
    let (size, upper, lower) = (10.37, 0.021, 0.003);
    let target = (size + upper, size + lower);
    for feature in [Feature::Hole, Feature::Shaft] {
        for m in list_closest(size, upper, lower, feature, false, 20, 2).unwrap() {
            let expected = absolute_error(m.size, &m.class, target);
            assert!(
                (m.error - expected).abs() < 1e-9,
                "{} {}: reported {} but recomputed {expected}",
                m.size,
                m.class,
                m.error
            );
        }
    }
}

#[test]
fn an_exact_match_reports_zero_error() {
    let found: Vec<Match> = list_closest(10.0, 0.0, -0.015, Feature::Hole, false, 4, 3).unwrap();
    for m in &found {
        assert_eq!(m.error, 0.0, "{} {} should be exact", m.size, m.class);
    }
}

#[test]
fn preferred_reports_its_distance_from_the_wanted_class() {
    for (tc, size) in [("H6", 10.0), ("K6", 30.0), ("n7", 80.0), ("M7", 638.0)] {
        for m in list_preferred(size, tc).unwrap() {
            assert_eq!(
                m.size, size,
                "list_preferred must not move the nominal size"
            );
            assert_eq!(m.tolerance(), limits(size, &m.class).unwrap());
            let expected = preferred_error(size, tc, &m.class);
            assert!(
                (m.error - expected).abs() < 1e-9,
                "{} reported {} but recomputed {expected}",
                m.class,
                m.error
            );
        }
    }
}

#[test]
fn a_preferred_class_is_its_own_zero_error_match() {
    assert_eq!(find_preferred(10.0, "H7").unwrap().error, 0.0);
    assert_eq!(find_preferred(30.0, "g6").unwrap().error, 0.0);
}
