impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product_l = 1;
        let mut product_r = 1;
        let mut ret = vec![1; nums.len()];

        for i in 0..nums.len() {
            let j = nums.len() - 1 - i;

            ret[i] *= product_l;
            ret[j] *= product_r;

            product_l *= nums[i];
            product_r *= nums[j];
        }

        ret
    }
}
