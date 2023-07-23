use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();

        for n in arr {
            *cnt.entry(n).or_insert(0) += 1;
        }

        match cnt.iter().filter(|(k, v)| k == v).max_by_key(|(&k, &v)| k) {
            Some((&k, &v)) => k,
            None => -1,
        }
    }
}
