# 2289. Steps to Make Array Non-decreasing
You are given a **0-indexed** integer array `nums`. In one step, **remove** all elements `nums[i]` where `nums[i - 1] > nums[i]` for all `0 < i < nums.length`.

Return *the number of steps performed until* `nums` *becomes a **non-decreasing** array*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,3,4,4,7,3,6,11,8,5,11]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The following are the steps performed:
- Step 1: [5,3,4,4,7,3,6,11,8,5,11] becomes [5,4,4,7,6,11,11]
- Step 2: [5,4,4,7,6,11,11] becomes [5,4,7,11,11]
- Step 3: [5,4,7,11,11] becomes [5,7,11,11]
[5,7,11,11] is a non-decreasing array. Therefore, we return 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,5,7,7,13]
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums is already a non-decreasing array. Therefore, we return 0.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut ret = 0;

        for &num in &nums {
            let mut x = 0;

            while stack.last().unwrap_or(&(i32::MAX, 0)).0 <= num {
                x = x.max(stack.pop().unwrap().1);
            }

            if stack.is_empty() {
                stack.push((num, 0));
            } else {
                stack.push((num, x + 1));
                ret = ret.max(x + 1);
            }
        }

        ret
    }
}
```
