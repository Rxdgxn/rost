mod ops;
use ops::*;

macro_rules! nth {
    ($src: expr, $it: expr) => {
        $src.chars().nth($it).unwrap()
    };
}

#[derive(Debug, Eq)]
pub struct Decimal {
    int: String,
    float: String
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.int == other.int && dezeroise(&self.float) == dezeroise(&other.float)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.int.cmp(&other.int).then(self.float.cmp(&other.float))
    }
}

impl PartialOrd for Decimal {
    fn gt(&self, other: &Self) -> bool {
        if nth!(self.int, 0) != '-' && nth!(other.int, 0) != '-' {
            if self.int != other.int {
                return self.int > other.int;
            }
            else {
                return self.float > other.float;
            }
        }
        else if nth!(self.int, 0) == '-' && nth!(other.int, 0) != '-' {
            return false;
        }
        else if nth!(self.int, 0) != '-' && nth!(other.int, 0) == '-' {
            return true;
        }
        else {
            return other.gt(self);
        }
    }
    fn lt(&self, other: &Self) -> bool {
        if nth!(self.int, 0) != '-' && nth!(other.int, 0) != '-' {
            if self.int != other.int {
                return self.int < other.int;
            }
            else {
                return self.float < other.float;
            }
        }
        else if nth!(self.int, 0) == '-' && nth!(other.int, 0) != '-' {
            return true;
        }
        else if nth!(self.int, 0) != '-' && nth!(other.int, 0) == '-' {
            return false;
        }
        else {
            return other.lt(self);
        }
    }
    fn ge(&self, other: &Self) -> bool {
        return self >= other;
    }
    fn le(&self, other: &Self) -> bool {
        return self <= other
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

#[test]
fn test6() {
    let d1 = Decimal::try_from("6.00");
    let d2 = Decimal::try_from("6.0");
    assert_eq!(d1 == d2, true);
}

#[test]
fn test7() {
    let d1 = Decimal::try_from("6.01");
    let d2 = Decimal::try_from("6.0");
    assert_eq!(d1 >= d2, true);
}