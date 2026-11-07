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
