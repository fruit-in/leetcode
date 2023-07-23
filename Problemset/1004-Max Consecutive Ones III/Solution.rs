impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut count0 = 0;
        let mut ret = 0;

        while r < nums.len() {
            if nums[r] == 0 {
                count0 += 1;
                while count0 > k {
                    if nums[l] == 0 {
                        count0 -= 1;
                    }
                    l += 1;
                }
            }

            r += 1;
            ret = ret.max(r - l);
        }

        ret as i32
    }
}
