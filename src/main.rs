use std::ops::{Add, Mul, Sub};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Decimal {
    int: String,
    float: String
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, _other: Self) -> Self {
        todo!()
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, _other: Self) -> Self {
        todo!()
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, _other: Self) -> Self {
        todo!()
    }
}

impl Decimal {
    fn try_from(input: &str) -> Option<Decimal> {
        let dots = input.matches('.').count();
        if dots != 1 {
            return None;
        }
        let idx = input.find('.').unwrap();
        if idx == input.len() - 1 {
            return None;
        }
        let decimal = Decimal { int: input[0..idx].to_string(), float: input[idx+1..input.len()].to_string() };
        return Some(decimal);
    }
}


fn main() {
    let d = Decimal::try_from("3.14");
    println!("{:?}", d);
}