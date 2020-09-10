struct Cashier {
    n: i32,
    discount: f64,
    customers: i32,
    prices: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut p = vec![0; 201];

        for i in 0..prices.len() {
            p[products[i] as usize] = prices[i];
        }

        Self {
            n: n,
            discount: discount as f64,
            customers: 0,
            prices: p,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut cost = 0;
        self.customers += 1;

        for i in 0..amount.len() {
            cost += self.prices[product[i] as usize] * amount[i];
        }

        match self.customers % self.n {
            0 => cost as f64 - (self.discount * cost as f64) / 100.0,
            _ => cost as f64,
        }
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
