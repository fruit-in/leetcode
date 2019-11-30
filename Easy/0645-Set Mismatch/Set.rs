use std::collections::HashSet;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = (1..=(nums.len() as i32)).collect::<HashSet<i32>>();
        let mut dup = 0;
        let mut miss = 0;

        for num in nums {
            if !set.remove(&num) {
                dup = num;
            }
        }

        miss = set.drain().next().unwrap();

        vec![dup, miss]
    }
}
