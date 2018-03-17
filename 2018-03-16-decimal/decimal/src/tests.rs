#[test]
fn ranges() {
    use digit_average::Range;
    let x: Range<u32> = Range::new(14, 20);
    assert_eq!(x.collect::<Vec<u32>>(), vec![14, 15, 16, 17, 18, 19]);

    let x: Range<i8> = Range::upper(5);
    assert_eq!(x.collect::<Vec<i8>>(), vec![0, 1, 2, 3, 4]);
}

#[test]
fn digit_sum() {
    use digit_average::div_sum;
    assert_eq!(div_sum(15, 73, 5, 10), (12, false));
    assert_eq!(div_sum(9, 4, 4, 10), (9, true));
    assert_eq!(div_sum(1, 1, 1, 10), (1, false));
}
