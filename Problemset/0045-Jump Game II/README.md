# 45. Jump Game II
You are given a **0-indexed** array of integers `nums` of length `n`. You are initially positioned at `nums[0]`.

Each element `nums[i]` represents the maximum length of a forward jump from index `i`. In other words, if you are at `nums[i]`, you can jump to any `nums[i + j]` where:

* `0 <= j <= nums[i]` and
* `i + j < n`

Return *the minimum number of jumps to reach* `nums[n - 1]`. The test cases are generated such that you can reach `nums[n - 1]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,1,1,4]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,0,1,4]
<strong>Output:</strong> 2
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* `0 <= nums[i] <= 1000`
* It's guaranteed that you can reach `nums[n - 1]`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut ret = 0;

        for k in 0..nums.len() {
            if k > i {
                i = j;
                ret += 1;
            }
            j = j.max(k + nums[k] as usize);
        }

        ret
    }
}
```
