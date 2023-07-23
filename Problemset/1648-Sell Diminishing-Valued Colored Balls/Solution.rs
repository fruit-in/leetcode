use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut orders = orders as i64;
        let mut count = HashMap::new();
        let mut ret = 0;

        for &balls in &inventory {
            count
                .entry(balls as i64)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let mut heap = count.into_iter().collect::<BinaryHeap<_>>();

        while orders > 0 {
            let (b0, c0) = heap.pop().unwrap();
            let (b1, c1) = heap.pop().unwrap_or((0, 0));

            if (b0 - b1) * c0 < orders {
                orders -= (b0 - b1) * c0;
                heap.push((b1, c0 + c1));
                ret = (ret + (b0 - b1) * (b0 + b1 + 1) * c0 / 2) % 1_000_000_007;
            } else {
                let (x, y) = (orders / c0, orders % c0);
                orders = 0;
                ret = (ret + (b0 * 2 - x + 1) * x * c0 / 2 + (b0 - x) * y) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
