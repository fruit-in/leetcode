impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min_num = nums.iter().min().unwrap();

        nums.iter().map(|&x| x - min_num).sum()
    }
}
