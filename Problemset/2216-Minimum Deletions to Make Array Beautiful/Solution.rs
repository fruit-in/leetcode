impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut beautiful = vec![];

        for i in 0..nums.len() {
            if beautiful.len() % 2 == 0 || *beautiful.last().unwrap_or(&-1) != nums[i] {
                beautiful.push(nums[i]);
            }
        }

        if beautiful.len() % 2 == 1 {
            beautiful.pop();
        }

        (nums.len() - beautiful.len()) as i32
    }
}
