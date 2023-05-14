use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0_i64;

        for i in 0..deliciousness.len() {
            count
                .entry(deliciousness[i])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for (&food0, c) in count.iter() {
            for i in 0..22 {
                let food1 = (1 << i) - food0;
                if food1 < food0 {
                    ret = (ret + c * count.get(&food1).unwrap_or(&0)) % 1_000_000_007;
                } else if food1 == food0 {
                    ret = (ret + c * (c - 1) / 2) % 1_000_000_007;
                } else {
                    break;
                }
            }
        }

        ret as i32
    }
}
