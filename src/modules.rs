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
