use gmp::mpq::Mpq;


pub fn expected_days_float(number_of_tablets: u64, proportion: f64) -> f64 {
    let n = number_of_tablets;
    let p = proportion;

    let mut sum = 0_f64;
    let mut product = 1_f64;

    for k in 1..n + 1 {
        product *= 1. / (1. + p * ((k - 1) as f64) / ((1. - p) * ((n - k + 1) as f64)));
        sum += (k as f64) / (1. + (1. - p) * ((n - k) as f64) / (p * (k as f64))) * product;
    }
    sum
}


pub fn expected_days_exact(number_of_tablets: u64, proportion: Mpq) -> Mpq {
    let n = number_of_tablets;
    let p = proportion;

    let mut sum: Mpq = 0.into();
    let mut product: Mpq = 1.into();

    for k in 1..n + 1 {
        product /= Mpq::from(1) +
            ((p.clone() * Mpq::from(k - 1)) / ((Mpq::from(1) - p.clone()) * Mpq::from(n - k + 1)));
        sum += Mpq::from(k) /
            (Mpq::from(1) +
                 (Mpq::from(1) - p.clone()) * Mpq::from(n - k) / (p.clone() * Mpq::from(k))) *
            product.clone();
    }
    sum
}
