use std::collections::HashMap;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = arr.len();
        let mut visited = vec![false; n];
        let mut groups = HashMap::new();
        let mut ret = 0;

        for i in 0..n {
            if !visited[i] {
                let mut j = i;
                groups.insert(i, vec![]);

                while !visited[j] {
                    visited[j] = true;
                    groups.get_mut(&i).unwrap().push(arr[j]);
                    j = (j + k) % n;
                }
            }
        }

        for mut group in groups.into_values() {
            let i = group.len() / 2;
            group.sort_unstable();

            for j in 0..group.len() {
                ret += (group[j] - group[i]).abs() as i64;
            }
        }

        ret
    }
}
