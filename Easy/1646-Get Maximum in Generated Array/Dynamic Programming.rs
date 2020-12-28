impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut nums = vec![0; n as usize + 1];

        for j in 1..nums.len() {
            nums[j] = match j / 2 {
                0 => 1,
                i if j % 2 == 0 => nums[i],
                i => nums[i] + nums[i + 1],
            };
        }

        nums.into_iter().max().unwrap()
    }
}
