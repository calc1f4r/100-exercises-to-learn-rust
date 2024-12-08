pub struct Order {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        assert!(!product_name.is_empty(), "Product name cannot be empty");
        assert!(
            product_name.len() <= 300,
            "Product name cannot be longer than 300 bytes"
        );
        assert!(quantity > 0, "Quantity must be greater than zero");
        assert!(unit_price > 0, "Unit price must be greater than zero");
        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        assert!(!product_name.is_empty(), "Product name cannot be empty");
        assert!(
            product_name.len() <= 300,
            "Product name cannot be longer than 300 bytes"
        );
        self.product_name = product_name;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        assert!(quantity > 0, "Quantity must be greater than zero");
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        assert!(unit_price > 0, "Unit price must be greater than zero");
        self.unit_price = unit_price;
    }
}
