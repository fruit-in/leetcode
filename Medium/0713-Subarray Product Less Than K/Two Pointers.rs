impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k < 2 {
            return 0;
        }

        let mut product = 1;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            product *= nums[j];
            while product >= k {
                product /= nums[i];
                i += 1;
            }
            ret += j - i + 1;
        }

        ret as i32
    }
}
