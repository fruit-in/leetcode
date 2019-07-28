impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut head = 0;
        let mut tail = nums.len() as i32 - 1;
        while head <= tail {
            if target == nums[(head + tail) as usize / 2] {
                return (head + tail) / 2;
            } else if target > nums[(head + tail) as usize / 2] {
                head = (head + tail) / 2 + 1;
            } else if target < nums[(head + tail) as usize / 2] {
                tail = (head + tail) / 2 - 1;
            }
        }
        -1
    }
}
