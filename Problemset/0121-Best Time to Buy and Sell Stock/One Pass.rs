impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min_price = std::i32::MAX;
        for n in prices {
            profit = profit.max(n - min_price);
            min_price = min_price.min(n);
        }
        profit
    }
}
