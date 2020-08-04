impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![0; prices.len()];

        for i in (0..prices.len()).rev() {
            while *stack.last().unwrap_or(&0) > prices[i] {
                stack.pop();
            }
            ret[i] = prices[i] - *stack.last().unwrap_or(&0);
            stack.push(prices[i]);
        }

        ret
    }
}
