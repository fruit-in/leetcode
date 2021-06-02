use std::collections::HashMap;

impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        let mut needs: HashMap<i32, Vec<i32>> = HashMap::new();
        nums.sort_unstable();

        for x in nums {
            if let Some(v) = needs.get_mut(&(x - 1)) {
                match v.pop() {
                    Some(1) => (),
                    Some(y) => needs.entry(x).or_insert(vec![]).push(y - 1),
                    None => needs.entry(x).or_insert(vec![]).push(k - 1),
                }
            } else if k > 1 {
                needs.entry(x).or_insert(vec![]).push(k - 1);
            }
        }

        needs.values().all(|v| v.is_empty())
    }
}
