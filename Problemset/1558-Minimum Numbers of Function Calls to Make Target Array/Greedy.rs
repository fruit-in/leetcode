impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut op0 = 0;
        let mut op1 = 0;

        for mut num in nums {
            let mut tmp = 0;

            while num > 0 {
                op0 += num % 2;
                tmp += 1;
                num /= 2;
            }
            op1 = op1.max(tmp - 1);
        }

        op0 + op1
    }
}
