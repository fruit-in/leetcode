struct ProductOfNumbers {
    products: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            products: vec![1],
        }
    }

    fn add(&mut self, num: i32) {
        match num {
            0 => self.products.truncate(1),
            _ => self.products.push(*self.products.last().unwrap() * num),
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let len = self.products.len();

        if k > len as i32 - 1 {
            return 0;
        }
        self.products[len - 1] / self.products[len - 1 - k as usize]
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
