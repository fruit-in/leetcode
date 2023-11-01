use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut changed = changed;
        let mut count = HashMap::new();
        let mut original = vec![];

        changed.sort_unstable();

        for num in &changed {
            if num % 2 == 0 && *count.get(&(num / 2)).unwrap_or(&0) > 0 {
                *count.get_mut(&(num / 2)).unwrap() -= 1;
                original.push(num / 2);
            } else {
                *count.entry(num).or_insert(0) += 1;
            }
        }

        if original.len() * 2 != changed.len() {
            original.clear();
        }

        original
    }
}
