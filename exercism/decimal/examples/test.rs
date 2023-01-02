use decimal::Decimal;

fn main() {
    decimal("0000.00000");
    decimal("000.0100000");
    // decimal("-0.1");
    // decimal("-000.01");
    decimal("-02.090100");
    // decimal("1.002400");
    // decimal("1.1000");
    // println!(
    //     "decimal(\"1.1\") + decimal(\"1.1\"): {:?}",
    //     decimal("1.1") + decimal("1.1")
    // );
    // println!(
    //     "decimal(\"1.11\") + decimal(\"1.1\"): {:?}",
    //     decimal("1.11") + decimal("1.1")
    // );

    // println!(
    //     r#"decimal("1.1") + decimal("-2.1"): {:?}"#,
    //     decimal("3.1") + decimal("-2.2")
    // );

    // println!(
    //     r#"decimal("3.1") * decimal("-2.123"): {:?}"#,
    //     decimal("3.1") * decimal("-2.123")
    // );
    // decimal(
    //     "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
    // );

    // println!(
    //     r#"decimal("-1.99") + decimal("-0.01"): {:?}"#,
    //     decimal("-1.99") + decimal("-0.01")
    // );

    // println!(
    //     r#"decimal("0") - decimal("1"): {:?}"#,
    //     decimal("0") - decimal("1")
    // );

    // println!(
    //     r#"decimal("5.5") + decimal("-6.5"): {:?}"#,
    //     decimal("5.5") + decimal("-6.5")
    // );
}

fn decimal(input: &str) -> Decimal {
    let dec = Decimal::try_from(input).unwrap();
    println!("Decimal::try_from({input}): {:?}", dec);
    dec
}
