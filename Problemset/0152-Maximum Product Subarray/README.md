# 152. Maximum Product Subarray
Given an integer array `nums`, find a subarray that has the largest product, and return *the product*.

The test cases are generated so that the answer will fit in a **32-bit** integer.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,-2,4]
<strong>Output:</strong> 6
<strong>Explanation:</strong> [2,3] has the largest product 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-2,0,-1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The result cannot be 2, because [-2,-1] is not a subarray.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* `-10 <= nums[i] <= 10`
* The product of any prefix or suffix of `nums` is **guaranteed** to fit in a **32-bit** integer.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ret = *nums.iter().max().unwrap();

        for slice in nums.split(|&num| num == 0) {
            if slice.iter().filter(|&&num| num < 0).count() % 2 == 1 {
                let i = slice.iter().position(|&num| num < 0).unwrap();
                let j = slice.iter().rposition(|&num| num < 0).unwrap();

                if i + 1 < slice.len() {
                    ret = ret.max(slice.iter().skip(i + 1).product());
                }
                if j > 0 {
                    ret = ret.max(slice.iter().take(j).product());
                }
            } else if !slice.is_empty() {
                ret = ret.max(slice.iter().product());
            }
        }

        ret
    }
}
```
