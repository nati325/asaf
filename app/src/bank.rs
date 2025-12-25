#[derive(Clone)]
pub struct Bank {
    pub name: String,
    pub balance: f64,
}

impl Bank {
    pub fn new(name: String, balance: f64) -> Self {
        Bank { name, balance }
    }

    pub fn print_info(&self) {
        println!("Bank Name: {}, Balance: {}", self.name, self.balance);
    }
}