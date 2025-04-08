# 862. Shortest Subarray with Sum at Least K
Given an integer array `nums` and an integer `k`, return *the length of the shortest non-empty **subarray** of* `nums` *with a sum of at least* `k`. If there is no such **subarray**, return `-1`.

A **subarray** is a **contiguous** part of an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1], k = 1
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2], k = 4
<strong>Output:</strong> -1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,-1,2], k = 3
<strong>Output:</strong> 3
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut prefix_sum = 0;
        let mut stack = vec![(0, -1)];
        let mut ret = i32::MAX;

        for i in 0..nums.len() {
            prefix_sum += nums[i] as i64;

            while stack.last().unwrap_or(&(i64::MIN, 0)).0 >= prefix_sum {
                stack.pop();
            }
            stack.push((prefix_sum, i as i32));

            let j = stack
                .binary_search(&(prefix_sum - k, i as i32))
                .unwrap_err();
            if j > 0 {
                ret = ret.min(i as i32 - stack[j - 1].1);
            }
        }

        if ret == i32::MAX {
            return -1;
        }

        ret
    }
}
```
