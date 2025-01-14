use std::collections::HashMap;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut diff = 0;
        let mut first_diff_index = HashMap::new();
        let mut ret = 0;

        for i in 0..hours.len() {
            diff += (hours[i] / 9) * 2 - 1;

            if diff > 0 {
                ret = i + 1;
            } else {
                if let Some(&j) = first_diff_index.get(&(diff - 1)) {
                    ret = ret.max(i - j);
                }
                if diff < 0 && !first_diff_index.contains_key(&diff) {
                    first_diff_index.insert(diff, i);
                }
            }
        }

        ret as i32
    }
}
