# 1437. Check If All 1's Are at Least Length K Places Away
Given an array `nums` of 0s and 1s and an integer `k`, return `True` if all 1's are at least `k` places away from each other, otherwise return `False`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/04/15/sample_1_1791.png)
<pre>
<strong>Input:</strong> nums = [1,0,0,0,1,0,0,1], k = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> Each of the 1s are at least 2 places away from each other.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/04/15/sample_2_1791.png)
<pre>
<strong>Input:</strong> nums = [1,0,0,1,0,1], k = 2
<strong>Output:</strong> false
<strong>Explanation:</strong> The second 1 and third 1 are only one apart from each other.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1], k = 0
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [0,1,0,1], k = 1
<strong>Output:</strong> true
</pre>

#### Constraints:
* `1 <= nums.length <= 10^5`
* `0 <= k <= nums.length`
* `nums[i]` is `0` or `1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_1 = -k - 1;

        for i in 0..nums.len() {
            if nums[i] == 1 {
                if i as i32 - prev_1 <= k {
                    return false;
                }
                prev_1 = i as i32;
            }
        }

        true
    }
}
```
