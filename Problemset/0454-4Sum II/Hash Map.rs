use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut map = HashMap::new();
        for num_a in &a {
            for num_b in &b {
                *map.entry(num_a + num_b).or_insert(0) += 1;
            }
        }
        for num_c in &c {
            for num_d in &d {
                ans += map.get(&-(num_c + num_d)).unwrap_or(&0);
            }
        }
        ans
    }
}
