//! Real PostgreSQL tests. Run explicitly with --ignored and DATABASE_URL.
#[cfg(test)]
mod tests {
    use rs_decimal::{BigInt, Decimal, NullDecimal};
    use sqlx::{Connection, Executor, PgConnection, Row};

    #[test]
    #[ignore = "requires a dedicated PostgreSQL server and DATABASE_URL"]
    fn real_numeric_roundtrips() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let url = std::env::var("DATABASE_URL")
                    .expect("DATABASE_URL is required; never silently skip integration tests");
                let mut conn = PgConnection::connect(&url).await.unwrap();
                conn.execute("CREATE TEMP TABLE decimal_cases (value NUMERIC)")
                    .await
                    .unwrap();
                let mut values = vec![
                    Decimal::default(),
                    Decimal::new_with_prec(0, 4),
                    Decimal::must_from_string("9007199254740993.00000000000000001"),
                    Decimal::must_from_string("-12345.6700"),
                    Decimal::new_with_prec(1, 16383),
                ];
                let dense = "9".repeat(131072) + "." + &"9".repeat(16383);
                values.push(Decimal::must_from_string(&dense));
                for d in &values {
                    conn.execute(sqlx::query("INSERT INTO decimal_cases VALUES ($1)").bind(d))
                        .await
                        .unwrap();
                    let got: Decimal = sqlx::query_scalar("SELECT $1::numeric")
                        .bind(d)
                        .fetch_one(&mut conn)
                        .await
                        .unwrap();
                    assert_eq!(&got, d);
                    assert_eq!(got.precision(), d.precision());
                    assert!(!got.is_nil());
                }
                let count: i64 = sqlx::query_scalar("SELECT count(*) FROM decimal_cases")
                    .fetch_one(&mut conn)
                    .await
                    .unwrap();
                assert_eq!(count, values.len() as i64);
                let nullable = [
                    NullDecimal::default(),
                    NullDecimal::new(Decimal::default()),
                    NullDecimal::new(Decimal::must_from_string("1.2300")),
                ];
                for n in &nullable {
                    let got: NullDecimal = sqlx::query_scalar("SELECT $1::numeric")
                        .bind(n)
                        .fetch_one(&mut conn)
                        .await
                        .unwrap();
                    assert_eq!(got.valid, n.valid);
                    assert_eq!(got.decimal, n.decimal);
                    if n.valid {
                        assert!(!got.decimal.is_nil());
                    } else {
                        assert!(got.decimal.is_nil());
                    }
                }
                let got: Vec<NullDecimal> = sqlx::query_scalar("SELECT $1::numeric[]")
                    .bind(&nullable[..])
                    .fetch_one(&mut conn)
                    .await
                    .unwrap();
                assert_eq!(got.len(), 3);
                assert!(!got[0].valid);
                assert_eq!(got[2].decimal.precision(), 4);
                let none: Option<Decimal> = sqlx::query_scalar("SELECT $1::numeric")
                    .bind(None::<Decimal>)
                    .fetch_one(&mut conn)
                    .await
                    .unwrap();
                assert!(none.is_none());
                assert!(
                    sqlx::query_scalar::<_, Decimal>("SELECT NULL::numeric")
                        .fetch_one(&mut conn)
                        .await
                        .is_err()
                );
                for special in ["NaN", "Infinity", "-Infinity"] {
                    assert!(
                        sqlx::query_scalar::<_, Decimal>("SELECT $1::text::numeric")
                            .bind(special)
                            .fetch_one(&mut conn)
                            .await
                            .is_err()
                    );
                }
                for value in [
                    Decimal::new_with_prec(1, 16384),
                    Decimal::new_from_big_int(&BigInt::from(10).pow(131072)),
                ] {
                    assert!(
                        sqlx::query_scalar::<_, Decimal>("SELECT $1::numeric")
                            .bind(value)
                            .fetch_one(&mut conn)
                            .await
                            .is_err()
                    );
                }
                // Simple-query protocol returns text rather than binary; preserve zero scale too.
                let rows = sqlx::raw_sql("SELECT 0.0000::numeric AS z, 123.4500::numeric AS n")
                    .fetch_all(&mut conn)
                    .await
                    .unwrap();
                let zero: Decimal = rows[0].try_get("z").unwrap();
                let number: Decimal = rows[0].try_get("n").unwrap();
                assert_eq!(zero.precision(), 4);
                assert_eq!(number.string_with_trailing_zeros(), "123.4500");
                assert!(
                    sqlx::query_scalar::<_, Decimal>("SELECT 1.25::float8")
                        .fetch_one(&mut conn)
                        .await
                        .is_err()
                );
                conn.close().await.unwrap();
            });
    }
}
