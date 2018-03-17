extern crate num_integer;
use self::num_integer::Integer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    pub fn try_from_char(c: char) -> Option<Digit> {
        match c {
            '0' => Some(Digit::Zero),
            '1' => Some(Digit::One),
            '2' => Some(Digit::Two),
            '3' => Some(Digit::Three),
            '4' => Some(Digit::Four),
            '5' => Some(Digit::Five),
            '6' => Some(Digit::Six),
            '7' => Some(Digit::Seven),
            '8' => Some(Digit::Eight),
            '9' => Some(Digit::Nine),
            _ => None,
        }
    }

    pub fn try_from_u64(num: u64) -> Option<Digit> {
        match num {
            0 => Some(Digit::Zero),
            1 => Some(Digit::One),
            2 => Some(Digit::Two),
            3 => Some(Digit::Three),
            4 => Some(Digit::Four),
            5 => Some(Digit::Five),
            6 => Some(Digit::Six),
            7 => Some(Digit::Seven),
            8 => Some(Digit::Eight),
            9 => Some(Digit::Nine),
            _ => None,
        }
    }
}

impl Into<u64> for Digit {
    fn into(self) -> u64 {
        match self {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Decimal(Vec<Digit>);

impl Into<Vec<Digit>> for Decimal {
    fn into(self) -> Vec<Digit> {
        self.0
    }
}

impl From<Vec<Digit>> for Decimal {
    fn from(digits: Vec<Digit>) -> Decimal {
        Decimal(digits)
    }
}

impl Decimal {
    pub fn try_from_str(s: &str) -> Option<Decimal> {
        // There should be a way to do this with map/fold, but I can't find it
        let mut v = Vec::new();
        for c in s.chars() {
            let d = Digit::try_from_char(c)?;
            v.push(d);
        }
        Some(v.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn sum_digits(&self) -> u64 {
        self.0
            .iter()
            .fold(0, |total, digit| total + <Digit as Into<u64>>::into(*digit))
    }

    pub fn average(&self) -> Decimal {
        // We'll think of this as a state (v, q, r)
        // v is the vector that we've figured out so far
        // q + r/n the remainder (times a suitable power of 10)
        // (where n is the original thing we're dividing by: the length of self)

        let sum = self.sum_digits();
        let n = self.len() as u64;
        let mut v: Vec<Digit> = Vec::new();

        // The first digit is a special case, so we'll do it outside of the loop
        if n == 0 {
            return v.into();
        }

        let (q, mut r) = sum.div_rem(&n);
        // We can unwrap because 0 < sum <= 9 * n < 10 * n
        v.push(Digit::try_from_u64(q).unwrap());

        while (v.len() as u64) <= n {
            println!("before before: q: {}, r: {}, n: {}", q, r, n);
            r *= 10;
            println!("before: q: {}, r: {}, n: {}", q, r, n);
            let (q, r_tmp) = r.div_rem(&n);
            r = r_tmp; // if we replaced r_tmp with r above, it would just shadow r
            println!("after: q: {}, r: {}, n: {}", q, r, n);
            v.push(Digit::try_from_u64(q).unwrap());
        }

        v.into()
    }
}
