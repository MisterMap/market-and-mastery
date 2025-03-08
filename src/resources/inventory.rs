use std::{collections::HashMap, fmt::Display};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum InventoryResource {
    Wheat,
}

impl Display for InventoryResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryResource::Wheat => write!(f, "Wheat"),
        }
    }
}

pub struct Inventory {
    pub items: HashMap<InventoryResource, i32>,
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Inventory:\n")?;
        for (resource, amount) in &self.items {
            write!(f, "{}: {}\n", resource, amount)?;
        }
        Ok(())
    }
}
impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    pub fn add(&mut self, resource: InventoryResource, amount: i32) {
        *self.items.entry(resource).or_insert(0) += amount;
    }

    pub fn remove(&mut self, resource: InventoryResource, amount: i32) -> i32 {
        let inventory_amount = self.items.entry(resource).or_insert(0);
        let current_amount = *inventory_amount;
        if current_amount >= amount {
            *inventory_amount -= amount;
            return amount;
        }
        self.items.remove(&resource);
        return current_amount;
    }

    pub fn move_full_inventory_from(&mut self, other: &mut Inventory) {
        for (resource, amount) in other.items.iter_mut() {
            self.add(*resource, *amount);
        }
        other.items.clear();
    }
}
