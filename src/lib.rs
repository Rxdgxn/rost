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

#[test]
fn test1() {
    let d1 = Decimal::try_from("1.0");
    let d2 = Decimal::try_from("2.0");
    assert_eq!(d1.unwrap() + d2.unwrap(), Decimal::try_from("3.0").unwrap());
}

#[test]
fn test2() {
    let d1 = Decimal::try_from("189.99");
    let d2 = Decimal::try_from("342.879");
    assert_eq!(d1.unwrap() + d2.unwrap(), Decimal::try_from("532.869").unwrap());
}

#[test]
fn test3() {
    let d1 = Decimal::try_from("12.9");
    let d2 = Decimal::try_from("3.05");
    assert_eq!(d1.unwrap() + d2.unwrap(), Decimal::try_from("15.95").unwrap());
}