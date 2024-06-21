impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut x, mut y) = (0, -prices[0] - fee);

        for i in 1..prices.len() {
            (x, y) = (x.max(y + prices[i]), y.max(x - prices[i] - fee));
        }

        x
    }
}
