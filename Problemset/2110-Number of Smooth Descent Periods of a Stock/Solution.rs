impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count = 1;
        let mut ret = 0;

        for i in 1..prices.len() {
            if prices[i] - prices[i - 1] == -1 {
                count += 1;
            } else {
                ret += count * (count + 1) / 2;
                count = 1;
            }
        }

        ret += count * (count + 1) / 2;

        ret
    }
}
