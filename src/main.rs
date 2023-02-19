mod ops;
use ops::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Decimal {
    int: String,
    float: String
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
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
    let d1 = Decimal::try_from("10.99");
    let d2 = Decimal::try_from("3.879");
    println!("{:?}", d1.unwrap() + d2.unwrap());
}