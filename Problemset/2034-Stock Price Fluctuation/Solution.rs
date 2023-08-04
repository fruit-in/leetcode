use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct StockPrice {
    prices: HashMap<i32, i32>,
    max_time: i32,
    max_prices: BinaryHeap<(i32, i32)>,
    min_prices: BinaryHeap<(Reverse<i32>, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        Self {
            prices: HashMap::new(),
            max_time: 0,
            max_prices: BinaryHeap::new(),
            min_prices: BinaryHeap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.prices.insert(timestamp, price);
        self.max_time = self.max_time.max(timestamp);
        self.max_prices.push((price, timestamp));
        self.min_prices.push((Reverse(price), timestamp));
    }

    fn current(&self) -> i32 {
        *self.prices.get(&self.max_time).unwrap()
    }

    fn maximum(&mut self) -> i32 {
        while let Some(&(p, t)) = self.max_prices.peek() {
            if *self.prices.get(&t).unwrap() != p {
                self.max_prices.pop();
            } else {
                return p;
            }
        }

        unimplemented!();
    }

    fn minimum(&mut self) -> i32 {
        while let Some(&(Reverse(p), t)) = self.min_prices.peek() {
            if *self.prices.get(&t).unwrap() != p {
                self.min_prices.pop();
            } else {
                return p;
            }
        }

        unimplemented!();
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */
