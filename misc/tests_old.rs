use fin_dec::*;

#[test]
fn digit_from_char() {
    assert_eq!(Digit::try_from_char('3').unwrap(), Digit::Three);
    assert_eq!(Digit::try_from_char('a'), None);
}

#[test]
fn digit_from_u64() {
    assert_eq!(Digit::try_from_u64(3).unwrap(), Digit::Three);
    assert_eq!(Digit::try_from_u64(15), None);
}

#[test]
fn decimal_from_str() {
    assert_eq!(
        Decimal::try_from_str("123").unwrap(),
        vec![Digit::One, Digit::Two, Digit::Three].into()
    );
    assert_eq!(Decimal::try_from_str("1223345a3453"), None);
}

#[test]
fn decimal_len() {
    assert_eq!(Decimal::try_from_str("5346").unwrap().len(), 4);
}

#[test]
fn sum_digits() {
    assert_eq!(Decimal::try_from_str("12345").unwrap().sum_digits(), 15);
}

#[test]
fn decimal_average() {
    println!("{:?}", Decimal::try_from_str("566").unwrap().average(),);
    assert_eq!(
        Decimal::try_from_str("45").unwrap().average(),
        Decimal::try_from_str("45").unwrap()
    );
    assert_eq!(
        Decimal::try_from_str("566").unwrap().average(),
        Decimal::try_from_str("567").unwrap()
    );
    assert_eq!(
        Decimal::try_from_str("23").unwrap().average(),
        Decimal::try_from_str("25").unwrap()
    );
}
