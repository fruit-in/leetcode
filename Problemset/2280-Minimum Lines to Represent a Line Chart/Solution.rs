impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        let mut stock_prices = stock_prices;
        let mut ret = (stock_prices.len() > 1) as i32;

        stock_prices.sort_unstable();

        for i in 1..stock_prices.len() - 1 {
            let (x0, y0) = (stock_prices[i - 1][0] as i64, stock_prices[i - 1][1] as i64);
            let (x1, y1) = (stock_prices[i][0] as i64, stock_prices[i][1] as i64);
            let (x2, y2) = (stock_prices[i + 1][0] as i64, stock_prices[i + 1][1] as i64);

            ret += ((y1 - y0) * (x2 - x1) != (y2 - y1) * (x1 - x0)) as i32;
        }

        ret
    }
}
