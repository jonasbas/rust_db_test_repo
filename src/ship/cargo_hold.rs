use anyhow::*;
use std::vec;

use crate::cargo::{cargo::Cargo, cargo_type::CargoType};

#[derive(Debug)]
pub struct CargoHold {
    size: isize,
    inventory: Vec<Cargo>,
}

impl CargoHold {
    pub fn new(size: isize, inventory: Vec<Cargo>) -> Self {
        CargoHold { size, inventory }
    }

    pub fn get_remaining_size(&self) -> isize {
        let amount_stored = self
            .inventory
            .iter()
            .map(|item| item.get_amount())
            .reduce(|acc, e| acc + e)
            .unwrap_or(0);

        if amount_stored > self.size {
            0
        } else {
            self.size - amount_stored
        }
    }

    fn contains_cargo_type(&self, cargo_type: CargoType) -> bool {
        self.inventory
            .iter()
            .map(|item| item.get_cargo_type())
            .any(|cargo| cargo == cargo_type)
    }

    pub fn add_cargo(&mut self, cargo_to_add: Cargo) -> Result<()> {
        if self.get_remaining_size() < cargo_to_add.get_amount() {
            return Err(anyhow!("The cargo hold is full."));
        }

        if !self.contains_cargo_type(cargo_to_add.get_cargo_type()) {
            self.inventory.push(cargo_to_add);
            return Ok(());
        }

        self.inventory.iter_mut().for_each(|item| {
            if item.is_cargo_type(cargo_to_add.get_cargo_type()) {
                item.increase_amount(cargo_to_add.get_amount());
            };
        });

        Ok(())
    }
}

impl Default for CargoHold {
    fn default() -> Self {
        CargoHold {
            size: 200,
            inventory: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cargo::{cargo::Cargo, cargo_type::CargoType};
    use crate::ship::cargo_hold::CargoHold;
    use std::assert_eq;

    #[test]
    fn get_remaining_size_empty_test() {
        let example_hold = CargoHold::default();

        assert_eq!(200, example_hold.get_remaining_size());
    }

    #[test]
    fn get_remaining_size_filled_test() {
        let example_cargo = Cargo::new(50, CargoType::Wood);
        let example_hold = CargoHold::new(200, vec![example_cargo]);

        assert_eq!(150, example_hold.get_remaining_size());
    }

    #[test]
    fn get_remaining_size_full_test() {
        let example_cargo = Cargo::new(200, CargoType::Wood);
        let example_hold = CargoHold::new(200, vec![example_cargo]);

        assert_eq!(0, example_hold.get_remaining_size());
    }

    #[test]
    fn get_remaining_size_edge_case_test() {
        let example_cargo = Cargo::new(500, CargoType::Wood);
        let example_hold = CargoHold::new(200, vec![example_cargo]);

        assert_eq!(0, example_hold.get_remaining_size());
    }

    #[test]
    fn cargo_add_full_test() {
        let example_cargo = Cargo::new(50, CargoType::Wood);
        let mut example_hold = CargoHold::new(200, vec![example_cargo]);

        let cargo_to_add = Cargo::new(100, CargoType::Wood);

        let result = example_hold.add_cargo(cargo_to_add);
        assert!(result.is_ok());

        let filtered_inventory = example_hold
            .inventory
            .iter()
            .filter(|item| item.is_cargo_type(CargoType::Wood))
            .next();

        assert!(filtered_inventory.is_some());
        assert_eq!(150, filtered_inventory.unwrap().get_amount())
    }

    #[test]
    fn cargo_add_oversized_test() {
        let example_cargo = Cargo::new(50, CargoType::Wood);
        let mut example_hold = CargoHold::new(100, vec![example_cargo]);

        let cargo_to_add = Cargo::new(100, CargoType::Wood);

        let result = example_hold.add_cargo(cargo_to_add);

        assert!(result.is_err());
    }
}
