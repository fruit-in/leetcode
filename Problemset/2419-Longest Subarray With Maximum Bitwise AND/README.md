# 2419. Longest Subarray With Maximum Bitwise AND
You are given an integer array `nums` of size `n`.

Consider a **non-empty** subarray from `nums` that has the **maximum** possible **bitwise AND**.

* In other words, let `k` be the maximum value of the bitwise AND of **any** subarray of `nums`. Then, only subarrays with a bitwise AND equal to `k` should be considered.

Return *the length of the **longest** such subarray*.

The bitwise AND of an array is the bitwise AND of all the numbers in it.

A **subarray** is a contiguous sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,3,2,2]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The maximum possible bitwise AND of a subarray is 3.
The longest subarray with that value is [3,3], so we return 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The maximum possible bitwise AND of a subarray is 4.
The longest subarray with that value is [4], so we return 1.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_num = nums[0];
        let mut count = 1;
        let mut ret = 1;

        for i in 1..nums.len() {
            if nums[i] > max_num {
                max_num = nums[i];
                count = 1;
                ret = 1;
            } else if nums[i] == max_num && nums[i] != nums[i - 1] {
                count = 1;
            } else if nums[i] == max_num {
                count += 1;
                ret = ret.max(count);
            }
        }

        ret
    }
}
```
