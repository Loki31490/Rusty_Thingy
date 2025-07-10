use egui::{Button, Ui};
use rust_decimal::Decimal;
use std::fmt;

pub struct ProductMte {
    pub id: u32,
    pub manufacturer: String,
    pub name: String,
    pub price: Decimal,
    pub quantity: u32,
    pub in_stock: bool,
}

impl fmt::Display for ProductMte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id: {}\nmanufaturer: {}\nname: {}\nprice: {}\nquantity: {}\nin_stock: {}\n",
            self.id, self.manufacturer, self.name, self.price, self.quantity, self.in_stock
        )
    }
}

pub struct MyButton {
    text: String,
    enabled: bool,
}

impl MyButton {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            enabled: true,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        ui.add_enabled(self.enabled, Button::new(self.text))
            .clicked()
    }
}
