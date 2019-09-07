impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            profit += 0.max(prices[i] - prices[i - 1]);
        }
        profit
    }
}
