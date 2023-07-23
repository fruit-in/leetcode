impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for (i, buy) in prices.iter().enumerate() {
            for sell in &prices[(i + 1)..] {
                if sell - buy > profit {
                    profit = sell - buy;
                }
            }
        }
        profit
    }
}
