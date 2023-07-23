# 238. Product of Array Except Self
Given an array `nums` of *n* integers where *n* > 1,  return an array `output` such that `output[i]` is equal to the product of all the elements of `nums` except `nums[i]`.

#### Example:
<pre>
<b>Input:</b> [1,2,3,4]
<b>Output:</b> [24,12,8,6]
</pre>

**Constraint:** It's guaranteed that the product of the elements of any prefix or suffix of the array (including the whole array) fits in a 32 bit integer.

**Note:** Please solve it **without division** and in O(*n*).

#### Follow up:
Could you solve it with constant space complexity? (The output array **does not** count as extra space for the purpose of space complexity analysis.)

## Solutions (Rust)

### 1. Solution
```Rust
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
```
