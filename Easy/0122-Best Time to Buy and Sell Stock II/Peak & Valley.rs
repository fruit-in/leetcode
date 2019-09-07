impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut i = 0;
        while i + 1 < prices.len() {
            while i + 1 < prices.len() && prices[i] >= prices[i + 1] {
                i += 1;
            }
            profit -= prices[i];
            while i + 1 < prices.len() && prices[i] <= prices[i + 1] {
                i += 1;
            }
            profit += prices[i];
        }
        profit
    }
}
