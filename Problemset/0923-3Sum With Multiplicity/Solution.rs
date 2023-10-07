use std::collections::HashMap;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut hashmap = HashMap::new();
        let mut ret = 0;

        for j in 0..arr.len() {
            ret = (ret + hashmap.get(&(target - arr[j])).unwrap_or(&0)) % 1_000_000_007;

            for i in 0..j {
                hashmap
                    .entry(arr[i] + arr[j])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }

        ret
    }
}
