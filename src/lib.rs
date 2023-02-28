mod ops;
use ops::*;

#[derive(Debug)]
pub struct Decimal {
    int: String,
    float: String
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.int == other.int && self.float == other.float
    }
}

impl PartialOrd for Decimal {
    fn gt(&self, _other: &Self) -> bool {
        todo!()
    }
    fn lt(&self, _other: &Self) -> bool {
        todo!()
    }
    fn ge(&self, _other: &Self) -> bool {
        todo!()
    }
    fn le(&self, _other: &Self) -> bool {
        todo!()
    }
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
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

#[test]
fn test4() {
    let d1 = Decimal::try_from("99.9");
    let d2 = Decimal::try_from("66.05");
    assert_eq!(d1.unwrap() + d2.unwrap(), Decimal::try_from("165.95").unwrap());
}

#[test]
fn test5() {
    let d1 = Decimal::try_from("99.9");
    let d2 = Decimal::try_from("6.05");
    assert_eq!(d1.unwrap() + d2.unwrap(), Decimal::try_from("105.95").unwrap());
}