use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut counts = vec![(0, -1)].into_iter().collect::<HashMap<_, _>>();
        let mut ret = 0;

        for i in 0..nums.len() as i32 {
            match nums[i as usize] {
                0 => count += 1,
                _ => count -= 1,
            }
            if !counts.contains_key(&count) {
                counts.insert(count, i);
            }
            ret = ret.max(i - counts.get(&count).unwrap());
        }

        ret
    }
}
