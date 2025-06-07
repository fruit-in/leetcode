impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums = grid.concat();
        let mid = nums.len() / 2;

        if nums.iter().any(|&y| y % x != nums[0] % x) {
            return -1;
        }

        nums.sort_unstable();

        nums.iter().map(|&y| (y - nums[mid]).abs() / x).sum()
    }
}
