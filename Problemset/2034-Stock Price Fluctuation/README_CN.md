# 2034. 股票价格波动
给你一支股票价格的数据流。数据流中每一条记录包含一个 **时间戳** 和该时间点股票对应的 **价格** 。

不巧的是，由于股票市场内在的波动性，股票价格记录可能不是按时间顺序到来的。某些情况下，有的记录可能是错的。如果两个有相同时间戳的记录出现在数据流中，前一条记录视为错误记录，后出现的记录 **更正** 前一条错误的记录。

请你设计一个算法，实现：

* **更新** 股票在某一时间戳的股票价格，如果有之前同一时间戳的价格，这一操作将 **更正** 之前的错误价格。
* 找到当前记录里 **最新股票价格** 。**最新股票价格** 定义为时间戳最晚的股票价格。
* 找到当前记录里股票的 **最高价格** 。
* 找到当前记录里股票的 **最低价格** 。

请你实现 `StockPrice` 类：

* `StockPrice()` 初始化对象，当前无股票价格记录。
* `void update(int timestamp, int price)` 在时间点 `timestamp` 更新股票价格为 `price` 。
* `int current()` 返回股票 **最新价格** 。
* `int maximum()` 返回股票 **最高价格** 。
* `int minimum()` 返回股票 **最低价格** 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["StockPrice", "update", "update", "current", "maximum", "update", "maximum", "update", "minimum"]
[[], [1, 10], [2, 5], [], [], [1, 3], [], [4, 2], []]
<strong>输出:</strong>
[null, null, null, 5, 10, null, 5, null, 2]
<strong>解释:</strong>
StockPrice stockPrice = new StockPrice();
stockPrice.update(1, 10); // 时间戳为 [1] ，对应的股票价格为 [10] 。
stockPrice.update(2, 5);  // 时间戳为 [1,2] ，对应的股票价格为 [10,5] 。
stockPrice.current();     // 返回 5 ，最新时间戳为 2 ，对应价格为 5 。
stockPrice.maximum();     // 返回 10 ，最高价格的时间戳为 1 ，价格为 10 。
stockPrice.update(1, 3);  // 之前时间戳为 1 的价格错误，价格更新为 3 。
                          // 时间戳为 [1,2] ，对应股票价格为 [3,5] 。
stockPrice.maximum();     // 返回 5 ，更正后最高价格为 5 。
stockPrice.update(4, 2);  // 时间戳为 [1,2,4] ，对应价格为 [3,5,2] 。
stockPrice.minimum();     // 返回 2 ，最低价格时间戳为 4 ，价格为 2 。
</pre>

#### 提示:
* <code>1 <= timestamp, price <= 10<sup>9</sup></code>
* `update`，`current`，`maximum` 和 `minimum` **总** 调用次数不超过 <code>10<sup>5</sup></code> 。
* `current`，`maximum` 和 `minimum` 被调用时，`update` 操作 **至少** 已经被调用过 **一次** 。

## 题解 (Rust)

### 1. 题解
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
