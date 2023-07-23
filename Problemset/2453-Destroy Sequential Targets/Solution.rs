use std::collections::HashMap;

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut max_count = 0;
        let mut ret = 0;

        for i in 0..nums.len() {
            match map.get_mut(&(nums[i] % space)) {
                Some((min_num, count)) => {
                    *min_num = nums[i].min(*min_num);
                    *count += 1;
                    if max_count < *count || (max_count == *count && ret > *min_num) {
                        max_count = *count;
                        ret = *min_num;
                    }
                }
                None if max_count == 0 || (max_count == 1 && ret > nums[i]) => {
                    max_count = 1;
                    ret = nums[i];
                    map.insert(nums[i] % space, (nums[i], 1));
                }
                None => {
                    map.insert(nums[i] % space, (nums[i], 1));
                }
            }
        }

        ret
    }
}
