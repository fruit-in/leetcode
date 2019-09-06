impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = std::i32::MAX;
        for n in prices {
            max_profit = max_profit.max(n - min_price);
            min_price = min_price.min(n);
        }
        max_profit
    }
}
