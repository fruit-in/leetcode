# 1567. Maximum Length of Subarray With Positive Product
Given an array of integers `nums`, find the maximum length of a subarray where the product of all its elements is positive.

A subarray of an array is a consecutive sequence of zero or more values taken out of that array.

Return *the maximum length of a subarray with positive product*.

#### Example 1:
<pre>
<b>Input:</b> nums = [1,-2,-3,4]
<b>Output:</b> 4
<b>Explanation:</b> The array nums already has a positive product of 24.
</pre>

#### Example 2:
<pre>
<b>Input:</b> nums = [0,1,-2,-3,-4]
<b>Output:</b> 3
<b>Explanation:</b> The longest subarray with positive product is [1,-2,-3] which has a product of 6.
Notice that we cannot include 0 in the subarray since that'll make the product 0 which is not positive.
</pre>

#### Example 3:
<pre>
<b>Input:</b> nums = [-1,-2,-3,0,1]
<b>Output:</b> 2
<b>Explanation:</b> The longest subarray with positive product is [-1,-2] or [-2,-3].
</pre>

#### Example 4:
<pre>
<b>Input:</b> nums = [-1,2]
<b>Output:</b> 1
</pre>

#### Example 5:
<pre>
<b>Input:</b> nums = [1,2,3,5,-6,4,0,10]
<b>Output:</b> 4
</pre>

#### Constraints:
* `1 <= nums.length <= 10^5`
* `-10^9 <= nums[i] <= 10^9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for v in nums.split(|&num| num == 0) {
            let mut tmp = v.len();

            if v.iter().filter(|&&x| x < 0).count() % 2 == 1 {
                let l_ne = v.iter().position(|&x| x < 0);
                let r_ne = v.iter().rev().position(|&x| x < 0);

                tmp -= l_ne.min(r_ne).unwrap() + 1;
            }

            ret = ret.max(tmp);
        }

        ret as i32
    }
}
```
