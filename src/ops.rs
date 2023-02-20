use std::mem::swap;
use std::ops::{Add, Mul, Sub};
use std::cmp::{min, max};
use crate::Decimal;

macro_rules! nth {
    ($src: expr, $it: expr) => {
        ($src.chars().nth($it).unwrap() as i32)
    };
}

fn update(src: String, it: usize, push: &str) -> String {
    let mut ret = String::new();
    for (i, c) in src.chars().enumerate() {
        if i == it {
            ret.push_str(push);
        }
        else {
            ret.push(c);
        }
    }
    ret
}

impl Add for Decimal {
    type Output = Self;
    // TODO: use macros to avoid repetition + handle '-'
    fn add(mut self, mut other: Self) -> Self {

        if self.float.len() < other.float.len() {
            swap(&mut self.float, &mut other.float);
        }
        let mut float = self.float;
        let mut fl_it = min(float.len(), other.float.len()) - 1;
        
        if self.int.len() < other.int.len() {
            swap(&mut self.int, &mut other.int);
        }
        let mut int = self.int;
        let mut int_it = max(int.len(), other.int.len()) - 1;
        let diff = max(int.len(), other.int.len()) - min(int.len(), other.int.len());

        let mut extra = 0;
        while fl_it > 0 {
            let mut to_replace = (nth!(float, fl_it) + nth!(other.float, fl_it) - 96 + extra) % 10;
            extra = (nth!(float, fl_it) + nth!(other.float, fl_it) - 96) / 10;
            if to_replace > 9 {
                to_replace %= 10;
                extra += 1;
            }
            float = update(float, fl_it, &to_replace.to_string());
            fl_it -= 1;
        }
        let mut to_replace = (nth!(float, fl_it) + nth!(other.float, fl_it) - 96 + extra) % 10;
        if to_replace > 9 {
            to_replace %= 10;
            extra += 1;
            float = update(float, 0, &(extra.to_string() + &to_replace.to_string()));
        }
        else {
            float = update(float, 0, &to_replace.to_string());
        }
        
        if diff == 0 {
            while int_it > 0 {
                let mut to_replace = (nth!(int, int_it) + nth!(other.int, int_it) - 96 + extra) % 10;
                extra = (nth!(int, int_it) + nth!(other.int, int_it) - 96) / 10;
                if to_replace > 9 {
                    to_replace %= 10;
                    extra += 1;
                }
                int = update(int, int_it, &to_replace.to_string());
                int_it -= 1;
            }
            let mut to_replace = (nth!(int, int_it) + nth!(other.int, int_it) - 96 + extra) % 10;
            if to_replace > 9 {
                to_replace %= 10;
                extra += 1;
                int = update(int, 0, &(extra.to_string() + &to_replace.to_string()));
            }
            else {
                int = update(int, 0, &to_replace.to_string());
            }
        }
        else {
            while int_it >= diff {
                let mut to_replace = (nth!(int, int_it) + nth!(other.int, int_it - diff) - 96 + extra) % 10;
                extra = (nth!(int, int_it) + nth!(other.int, int_it - diff) - 96) / 10;
                if to_replace > 9 {
                    to_replace %= 10;
                    extra += 1;
                }
                int = update(int, int_it, &to_replace.to_string());
                int_it -= 1;
            }
            while int_it > 0 {
                let mut to_replace = (nth!(int, int_it) - 48) % 10 + extra;
                extra = (nth!(int, int_it) - 48) / 10;
                if to_replace > 9 {
                    to_replace %= 10;
                    extra += 1;
                }
                int = update(int, int_it, &to_replace.to_string());
                int_it -= 1;
            }
            let mut to_replace = (nth!(int, int_it) - 48) % 10 + extra;
            if to_replace > 9 {
                to_replace %= 10;
                extra += 1;
                int = update(int, 0, &(extra.to_string() + &to_replace.to_string()));
            }
            else {
                int = update(int, 0, &to_replace.to_string());
            }
        }
        
        Self {
            int,
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