impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .min_by_key(|num| (num.abs(), -num))
            .unwrap()
    }
}
