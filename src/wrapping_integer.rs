use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct WrappingInteger<T> {
    max: T,
    min: T,
    value: T
}

impl<T> WrappingInteger<T> 
    where T: Add,
    <T as Add>::Output: PartialOrd<T>,
    <T as Sub>::Output: PartialOrd<T>,
    T: AddAssign, 
    T: Sub <Output = T>,
    T: SubAssign,
    T: Copy,
    T: std::cmp::PartialOrd
{
    pub fn new(max: T, min: T, start_value: T) -> WrappingInteger<T> {
        WrappingInteger {max, min, value: start_value}
    }

    pub fn add(&mut self, amount: T) {
        if self.value > self.max - amount {
            self.value = self.min;
        }
        else {
            self.value += amount;
        }
    }

    pub fn subtract(&mut self,amount: T) {
        if self.value < amount || self.value - amount > self.value {
            self.value = self.max;
        }
        else {
            self.value -= amount;
        }
    }

    pub fn get_value(self) -> T {
        self.value
    }

    pub fn set_value(&mut self, new_val: T) {
        self.value = new_val;
    }
}