use std::fmt;
use std::ops::{Add, Sub};


#[derive(Clone, Eq, PartialEq)]
pub struct Dollars {
    pub cents: u64,
    pub year: u32,
}

impl Dollars {
    pub fn new(dollars: u32, cents: u32, year: u32) -> Self {
        Dollars {
            cents: (dollars as u64 * 100) + (cents as u64),
            year,
        }
    }

    pub fn tax(&self, rate: f32) -> Dollars {
        Dollars {
            cents: (self.cents as f32 * rate) as u64,
            year: self.year
        }
    }
}

impl fmt::Debug for Dollars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${:?}.{:?} in {:?} money", self.cents / 100, self.cents % 100, self.year)
    }

}


impl Add for Dollars {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.year, other.year);
        Dollars{
            cents: self.cents + other.cents,
            year: self.year
        }
    }
}

impl Sub for Dollars {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.year, other.year);
        Dollars{
            cents: self.cents - other.cents,
            year: self.year
        }
    }
}

