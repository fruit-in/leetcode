# 2034. Stock Price Fluctuation
You are given a stream of **records** about a particular stock. Each record contains a **timestamp** and the corresponding **price** of the stock at that timestamp.

Unfortunately due to the volatile nature of the stock market, the records do not come in order. Even worse, some records may be incorrect. Another record with the same timestamp may appear later in the stream **correcting** the price of the previous wrong record.

Design an algorithm that:

* **Updates** the price of the stock at a particular timestamp, **correcting** the price from any previous records at the timestamp.
* Finds the **latest price** of the stock based on the current records. The **latest price** is the price at the latest timestamp recorded.
* Finds the **maximum price** the stock has been based on the current records.
* Finds the **minimum price** the stock has been based on the current records.

Implement the `StockPrice` class:

* `StockPrice()` Initializes the object with no price records.
* `void update(int timestamp, int price)` Updates the `price` of the stock at the given `timestamp`.
* `int current()` Returns the **latest price** of the stock.
* `int maximum()` Returns the **maximum price** of the stock.
* `int minimum()` Returns the **minimum price** of the stock.

#### Example 1:
<pre>
<strong>Input:</strong>
["StockPrice", "update", "update", "current", "maximum", "update", "maximum", "update", "minimum"]
[[], [1, 10], [2, 5], [], [], [1, 3], [], [4, 2], []]
<strong>Output:</strong>
[null, null, null, 5, 10, null, 5, null, 2]
<strong>Explanation:</strong>
StockPrice stockPrice = new StockPrice();
stockPrice.update(1, 10); // Timestamps are [1] with corresponding prices [10].
stockPrice.update(2, 5);  // Timestamps are [1,2] with corresponding prices [10,5].
stockPrice.current();     // return 5, the latest timestamp is 2 with the price being 5.
stockPrice.maximum();     // return 10, the maximum price is 10 at timestamp 1.
stockPrice.update(1, 3);  // The previous timestamp 1 had the wrong price, so it is updated to 3.
                          // Timestamps are [1,2] with corresponding prices [3,5].
stockPrice.maximum();     // return 5, the maximum price is 5 after the correction.
stockPrice.update(4, 2);  // Timestamps are [1,2,4] with corresponding prices [3,5,2].
stockPrice.minimum();     // return 2, the minimum price is 2 at timestamp 4.
</pre>

#### Constraints:
* <code>1 <= timestamp, price <= 10<sup>9</sup></code>
* At most <code>10<sup>5</sup></code> calls will be made **in total** to `update`, `current`, `maximum`, and `minimum`.
* `current`, `maximum`, and `minimum` will be called **only after** `update` has been called **at least once**.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
