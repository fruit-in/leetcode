impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let m = *nums.iter().max().unwrap();
        if nums.iter().all(|&x| 2 * x <= m || x == m) {
            nums.iter().position(|&x| x == m).unwrap() as i32
        } else {
            -1
        }
    }
}
