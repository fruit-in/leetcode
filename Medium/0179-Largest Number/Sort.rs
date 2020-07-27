impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|num| num.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| (b.to_owned() + a).cmp(&(a.to_owned() + b)));

        if nums.len() > 0 && nums[0] == 0.to_string() {
            return 0.to_string();
        }
        nums.concat()
    }
}
