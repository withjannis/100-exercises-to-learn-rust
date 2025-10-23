// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    fn verify_pn(product_name: &String) {
        if product_name.is_empty() {
            panic!("empty!")
        }
        if product_name.len() > 300 {
            panic!("300!")
        }
    }
    fn verify_q(quantity: &i32) {
        if *quantity <= 0 {
            panic!("0!")
        }
    }
    fn verify_up(unit_price: &i32) {
        if *unit_price <= 0 {
            panic!("0!")
        }
    }

    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
        Order::verify_pn(&product_name);
        Order::verify_q(&quantity);
        Order::verify_up(&unit_price);


        Order {
            product_name,
            quantity,
            unit_price,
        }
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }
    pub fn total(&self) -> i32 {
        return self.quantity * self.unit_price;
    }

    pub fn set_product_name(&mut self, x: String) -> (){
        Order::verify_pn(&x);
        self.product_name = x;
    }

    pub fn set_quantity(&mut self, x: i32) -> (){
        Order::verify_q(&x);
        self.quantity = x;
    }
    pub fn set_unit_price(&mut self, x: i32) -> (){
        Order::verify_up(&x);
        self.unit_price = x;
    }

}

