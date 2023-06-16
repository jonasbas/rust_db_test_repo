use anyhow::{anyhow, Ok, Result};

use super::cargo_type::CargoType;

#[derive(Debug)]
pub struct Cargo {
    amount: isize,
    cargo_type: CargoType,
}

impl Cargo {
    pub fn new(amount: isize, cargo_type: CargoType) -> Self {
        Cargo { amount, cargo_type }
    }

    pub fn get_amount(&self) -> isize {
        self.amount
    }

    pub fn get_cargo_type(&self) -> CargoType {
        self.cargo_type.clone()
    }

    pub fn is_cargo_type(&self, cargo_type: CargoType) -> bool {
        self.cargo_type == cargo_type
    }

    pub fn increase_amount(&mut self, amount: isize) -> isize {
        self.amount += amount;
        self.amount
    }

    pub fn decrease_amount(&mut self, amount: isize) -> Result<isize> {
        if amount > self.amount {
            return Err(anyhow!("Trying to remove more than exists."));
        }
        self.amount -= amount;

        Ok(self.amount)
    }

    pub fn split_cargo(&mut self, amount: isize) -> Result<Cargo> {
        self.decrease_amount(amount)?;

        Ok(Cargo::new(amount, self.get_cargo_type()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cargo::cargo_type::CargoType;
    use std::assert_eq;

    #[test]
    fn decrease_below_zero_test() {
        let mut cargo = Cargo::new(50, CargoType::Wood);
        let result = cargo.decrease_amount(60);

        assert!(result.is_err());
    }

    #[test]
    fn decrease_amount_test() {
        let mut cargo = Cargo::new(50, CargoType::Wood);
        let result = cargo.decrease_amount(40);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn increase_amount_test() {
        let mut cargo = Cargo::new(50, CargoType::Wood);
        let result = cargo.increase_amount(40);

        assert_eq!(result, 90);
    }

    #[test]
    fn split_cargo_test() {
        let mut cargo = Cargo::new(50, CargoType::Wood);
        let split_cargo = cargo.split_cargo(25);

        assert!(split_cargo.is_ok());
        assert_eq!(25, split_cargo.unwrap().amount);
        assert_eq!(25, cargo.amount);
    }

    #[test]
    fn split_cargo_err_test() {
        let mut cargo = Cargo::new(50, CargoType::Wood);
        let split_cargo = cargo.split_cargo(100);

        assert!(split_cargo.is_err());
    }
}
