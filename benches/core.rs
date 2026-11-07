//! Dependency-free, in-process microbenchmarks. Run with `cargo bench --bench core`.
//! Inputs and expected outputs are prepared before timing; every family first
//! checks correctness. Timing includes result allocation/drop but no JSONL or I/O.
use rs_decimal::{Decimal, RoundingMode};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

fn measure<T>(name: &str, iterations: usize, mut operation: impl FnMut() -> T) {
    // Time-based warmup avoids measuring CPU ramp-up as a parsing regression.
    // Calibrate a roughly 50 ms sample, with bounded iteration counts.
    let warmup = Instant::now();
    let mut warmup_count = 0u128;
    while warmup.elapsed() < Duration::from_millis(100) {
        for _ in 0..iterations {
            black_box(operation());
        }
        warmup_count += iterations as u128;
    }
    let iterations =
        (warmup_count * 50_000_000 / warmup.elapsed().as_nanos()).clamp(100, 2_000_000) as usize;
    let mut samples = Vec::new();
    for _ in 0..7 {
        let started = Instant::now();
        for _ in 0..iterations {
            black_box(operation());
        }
        samples.push(started.elapsed().as_nanos() as f64 / iterations as f64);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "{name},{iterations},{:.2},{:.2},{:.2}",
        samples[3], samples[0], samples[6]
    );
}

fn main() {
    println!("operation,iterations,median_ns,min_ns,max_ns");
    copy_paths();
    ownership_paths();
    for (name, input, expected) in [
        (
            "parse_money",
            "12345.6700".to_owned(),
            "12345.67".to_owned(),
        ),
        (
            "parse_integer",
            "123456789012345678".to_owned(),
            "123456789012345678".to_owned(),
        ),
        (
            "parse_exponent",
            "-123456789.125e-4".to_owned(),
            "-12345.6789125".to_owned(),
        ),
        ("parse_large", "9".repeat(1000), "9".repeat(1000)),
    ] {
        assert_eq!(input.parse::<Decimal>().unwrap().string(), expected);
        measure(name, 20_000, || {
            black_box(&input).parse::<Decimal>().unwrap()
        });
    }
    let left: Decimal = "12345.6700".parse().unwrap();
    let same: Decimal = "98.7600".parse().unwrap();
    let mixed: Decimal = "98.76".parse().unwrap();
    assert_eq!(left.add(&same).string_with_trailing_zeros(), "12444.4300");
    assert_eq!(left.add(&mixed).string_with_trailing_zeros(), "12444.4300");
    measure("add_same_scale", 100_000, || {
        black_box(&left).add(black_box(&same))
    });
    measure("add_mixed_scale", 100_000, || {
        black_box(&left).add(black_box(&mixed))
    });
    assert_eq!(left.mul_exact(&Decimal::new(2)).string(), "24691.34");
    let two = Decimal::new(2);
    measure("multiply_exact", 100_000, || {
        black_box(&left).mul_exact(black_box(&two))
    });
    assert_eq!(
        left.quo_with_prec(&two, 2, RoundingMode::HalfEven).string(),
        "6172.84"
    );
    measure("divide_rounded", 20_000, || {
        black_box(&left).quo_with_prec(black_box(&two), 2, RoundingMode::HalfEven)
    });
    assert_eq!(left.string(), "12345.67");
    measure("format_money", 100_000, || black_box(&left).string());
    let large: Decimal = ("9".repeat(1000) + ".12300").parse().unwrap();
    assert_eq!(large.string(), "9".repeat(1000) + ".123");
    measure("format_large", 20_000, || black_box(&large).string());
}

// Identical inputs can also run on the pre-optimization library.
fn copy_paths() {
    for digits in [6, 1000] {
        let coefficient: rs_decimal::BigInt = "7".repeat(digits).parse().unwrap();
        let a = Decimal::new_from_big_int_with_prec(&coefficient, 2);
        let b = Decimal::new_with_prec(123, 2);
        assert_eq!(a.safe_add(&b).big_int(), &coefficient + 123);
        assert_eq!(a.safe_sub(&b).big_int(), &coefficient - 123);
        measure(&format!("safe_add_{digits}"), 1000, || {
            black_box(&a).safe_add(black_box(&b))
        });
        measure(&format!("safe_sub_{digits}"), 1000, || {
            black_box(&a).safe_sub(black_box(&b))
        });
        let lower = Decimal::new_from_big_int_with_prec(&coefficient, 0);
        for (order, left, right) in [("left", &lower, &a), ("right", &a, &lower)] {
            let aligned_left =
                left.big_int() * rs_decimal::BigInt::from(10).pow((2 - left.precision()) as u32);
            let aligned_right =
                right.big_int() * rs_decimal::BigInt::from(10).pow((2 - right.precision()) as u32);
            assert_eq!(left.add(right).big_int(), &aligned_left + &aligned_right);
            assert_eq!(left.sub(right).big_int(), &aligned_left - &aligned_right);
            measure(&format!("mixed_add_{order}_{digits}"), 1000, || {
                black_box(left).add(black_box(right))
            });
            measure(&format!("mixed_sub_{order}_{digits}"), 1000, || {
                black_box(left).sub(black_box(right))
            });
        }
        for (shift, scale) in [("positive", 0), ("zero", 2), ("negative", 4)] {
            let numerator = Decimal::new_from_big_int_with_prec(&coefficient, scale);
            let divisor = Decimal::new(3);
            let factor = rs_decimal::BigInt::from(10).pow(scale.abs_diff(2) as u32);
            let expected = if scale <= 2 {
                (&coefficient * factor) / 3
            } else {
                &coefficient / (factor * 3)
            };
            assert_eq!(
                numerator
                    .quo_with_prec(&divisor, 2, RoundingMode::Down)
                    .big_int(),
                expected
            );
            measure(&format!("divide_{shift}_{digits}"), 1000, || {
                black_box(&numerator).quo_with_prec(black_box(&divisor), 2, RoundingMode::Down)
            });
        }
        let negative = a.neg();
        let expected_text = format!("-{}.{}", "7".repeat(digits - 2), "77");
        assert_eq!(negative.string(), expected_text);
        measure(&format!("format_negative_{digits}"), 1000, || {
            black_box(&negative).string()
        });
        let scientific = negative.format_go(rs_decimal::GoFormat {
            verb: 'e',
            precision: Some(2),
            ..Default::default()
        });
        assert_eq!(scientific, format!("-7.78e+{:02}", digits - 3));
        measure(&format!("format_scientific_{digits}"), 1000, || {
            black_box(&negative).format_go(rs_decimal::GoFormat {
                verb: 'e',
                precision: Some(2),
                ..Default::default()
            })
        });
    }
}

// Both pipelines include the same initial clone to supply an owned input.
fn ownership_paths() {
    for digits in [6, 1000] {
        let coefficient: rs_decimal::BigInt = "7".repeat(digits).parse().unwrap();
        let start = Decimal::try_from_coefficient(coefficient.clone(), 2).unwrap();
        let rhs = Decimal::new_with_prec(123, 2);
        let borrowed = || {
            start
                .clone()
                .add(&rhs)
                .sub(&rhs)
                .rescale(2, RoundingMode::Unnecessary)
        };
        let owned = || {
            start
                .clone()
                .add_owned(&rhs)
                .sub_owned(&rhs)
                .into_rescaled(2, RoundingMode::Unnecessary)
        };
        for result in [borrowed(), owned()] {
            assert_eq!(result.coefficient(), &coefficient);
            assert_eq!(result.scale(), 2);
            assert!(!result.is_nil());
        }
        measure(&format!("pipeline_borrowed_{digits}"), 1000, || {
            black_box(&start)
                .clone()
                .add(black_box(&rhs))
                .sub(black_box(&rhs))
                .rescale(2, RoundingMode::Unnecessary)
        });
        measure(&format!("pipeline_owned_{digits}"), 1000, || {
            black_box(&start)
                .clone()
                .add_owned(black_box(&rhs))
                .sub_owned(black_box(&rhs))
                .into_rescaled(2, RoundingMode::Unnecessary)
        });
        assert_eq!(start.big_int(), *start.coefficient());
        assert_eq!(start.clone().into_coefficient(), coefficient);
        measure(&format!("coefficient_copy_{digits}"), 1000, || {
            black_box(&start).big_int()
        });
        measure(&format!("coefficient_borrow_{digits}"), 1000, || {
            black_box(&start).coefficient()
        });
        // Includes cloning the input; the move itself does not copy digits.
        measure(
            &format!("coefficient_clone_then_move_{digits}"),
            1000,
            || black_box(&start).clone().into_coefficient(),
        );
    }
}
