use std::collections::HashMap;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut count = HashMap::new();
        let mut min_fruit = i64::MAX;
        let mut swap = vec![];
        let mut ret = 0;

        for i in 0..basket1.len() {
            *count.entry(basket1[i]).or_insert(0_i32) += 1;
            *count.entry(basket2[i]).or_insert(0) -= 1;
            min_fruit = min_fruit.min(basket1[i].min(basket2[i]) as i64);
        }

        for (k, v) in count.into_iter() {
            if v % 2 != 0 {
                return -1;
            }

            swap.append(&mut vec![k as i64; v.abs() as usize / 2]);
        }

        swap.sort_unstable();

        swap.iter()
            .take(swap.len() / 2)
            .map(|&cost| cost.min(min_fruit * 2))
            .sum()
    }
}
