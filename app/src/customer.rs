use crate::bank::Bank;

#[derive(Clone)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub bank: Bank,
}

impl Customer {
    pub fn print_info(&self) {
        println!("Customer Name: {}, Age: {}, ID: {}, Bank: {}", self.name, self.age, self.id, self.bank.name);
    }

    pub fn new(name: String, age: u32, id: u32, bank: Bank) -> Self {
        Customer { id, name, age, bank }
    }
}