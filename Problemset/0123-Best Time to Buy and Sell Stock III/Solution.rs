impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_price = i32::MIN;
        let mut once_max = vec![0; prices.len()];
        let mut ret = 0;

        for i in 1..prices.len() {
            min_price = min_price.min(prices[i]);
            once_max[i] = (prices[i] - min_price).max(once_max[i - 1]);
        }
        for i in (0..prices.len()).rev() {
            max_price = max_price.max(prices[i]);
            ret = ret.max(max_price - prices[i] + once_max[i]);
        }

        ret
    }
}
