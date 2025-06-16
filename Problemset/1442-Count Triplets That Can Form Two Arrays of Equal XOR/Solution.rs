use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut prefix_xor = vec![0; arr.len() + 1];
        let mut prefix_xor_indices = HashMap::from([(0, vec![0])]);
        let mut ret = 0;

        for k in 0..arr.len() {
            prefix_xor[k + 1] = prefix_xor[k] ^ arr[k];
            for &i in prefix_xor_indices
                .get(&prefix_xor[k + 1])
                .unwrap_or(&vec![])
            {
                ret += k - i;
            }
            prefix_xor_indices
                .entry(prefix_xor[k + 1])
                .or_insert(vec![])
                .push(k + 1);
        }

        ret as i32
    }
}
