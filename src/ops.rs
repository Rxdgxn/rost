use std::mem::swap;
use std::ops::{Add, Mul, Sub};
use std::cmp::{min};
use crate::Decimal;


impl Add for Decimal {
    type Output = Self;
    
    fn add(mut self, mut other: Self) -> Self {
        if self.float.len() < other.float.len() {
            swap(&mut self.float, &mut other.float);
        }
        let mut float = self.float;
        let mut fl_it = min(float.len(), other.float.len()) - 1;

        let mut extra = 0;
        while fl_it > 0 {
            let mut to_replace = ((float.chars().nth(fl_it).unwrap() as i32) + (other.float.chars().nth(fl_it).unwrap() as i32) - 96) % 10 + extra;
            extra = ((float.chars().nth(fl_it).unwrap() as i32) + (other.float.chars().nth(fl_it).unwrap() as i32) - 96) / 10;
            if to_replace > 9 {
                to_replace %= 10;
                extra += 1;
            }
            float = float.replace(float.chars().nth(fl_it).unwrap(), &to_replace.to_string());
            fl_it -= 1;
        }
        let mut to_replace = ((float.chars().nth(fl_it).unwrap() as i32) + (other.float.chars().nth(fl_it).unwrap() as i32) - 96) % 10 + extra;
        if to_replace > 9 {
            to_replace %= 10;
            extra += 1;
            float = float.replace(float.chars().nth(0).unwrap(), &(extra.to_string() + &to_replace.to_string()));
        }
        else {
            float = float.replace(float.chars().nth(0).unwrap(), &to_replace.to_string());
        }
        
        
        Self {
            int: self.int,
            float
        }
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