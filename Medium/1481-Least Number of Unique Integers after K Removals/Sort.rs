use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut counter = HashMap::new();
        for x in arr {
            *counter.entry(x).or_insert(0) += 1;
        }

        let mut counter = counter.values().collect::<Vec<_>>();
        counter.sort_unstable();

        for i in 0..=counter.len() {
            if k == 0 {
                return (counter.len() - i) as i32;
            } else if k < 0 {
                return (counter.len() - i) as i32 + 1;
            }
            k -= counter[i];
        }

        0
    }
}
