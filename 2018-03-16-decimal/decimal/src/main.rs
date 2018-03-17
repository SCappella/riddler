mod digit_average;
mod tests;

use digit_average::AvgDec;

fn main() {
    println!("0, 1"); // for completeness
    for x in AvgDec::new(10_u32) {
        println!("{}, {}", x.0, x.1);
    }
}
