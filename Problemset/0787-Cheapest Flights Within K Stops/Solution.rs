use std::collections::VecDeque;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let k = k as usize;
        let mut to_cities = vec![vec![]; n];
        let mut deque = VecDeque::from([(src, 0)]);
        let mut min_prices = vec![vec![i32::MAX; n]; k + 2];
        let mut ret = -1;
        min_prices[0][src] = 0;

        for f in &flights {
            to_cities[f[0] as usize].push((f[1] as usize, f[2]));
        }

        while let Some((from, stops)) = deque.pop_front() {
            if stops > k {
                break;
            }

            for &(to, price) in &to_cities[from] {
                if min_prices[stops][from] + price < min_prices[stops + 1][to] {
                    min_prices[stops + 1][to] = min_prices[stops][from] + price;
                    deque.push_back((to, stops + 1));
                }
            }
        }

        for i in 0..=k + 1 {
            if min_prices[i][dst] != i32::MAX && (ret == -1 || min_prices[i][dst] < ret) {
                ret = min_prices[i][dst];
            }
        }

        ret
    }
}
