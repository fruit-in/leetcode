impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut m = 0;
        for n in nums {
            if n == 1 {
                i += 1;
            } else {
                m = m.max(i);
                i = 0;
            }
        }
        m.max(i)
    }
}
