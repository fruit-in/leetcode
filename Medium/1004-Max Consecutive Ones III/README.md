# 1004. Max Consecutive Ones III
Given a binary array `nums` and an integer `k`, return *the maximum number of consecutive* `1`*'s in the array if you can flip at most* `k` `0`'s.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> [1,1,1,0,0,1,1,1,1,1,1]
Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
<strong>Output:</strong> 10
<strong>Explanation:</strong> [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums[i]` is either `0` or `1`.
* `0 <= k <= nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
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
```
