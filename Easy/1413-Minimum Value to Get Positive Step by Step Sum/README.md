# 1413. Minimum Value to Get Positive Step by Step Sum
Given an array of integers `nums`, you start with an initial **positive** value *startValue*.

In each iteration, you calculate the step by step sum of *startValue* plus elements in `nums` (from left to right).

Return the minimum **positive** value of *startValue* such that the step by step sum is never less than 1.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-3,2,-3,4,2]
<strong>Output:</strong> 5
<strong>Explanation:</strong> If you choose startValue = 4, in the third iteration your step by step sum is less than 1.
                <strong>step by step sum
                startValue = 4 | startValue = 5 | nums</strong>
                  (4 <strong>-3</strong> ) = 1  | (5 <strong>-3</strong> ) = 2    |  -3
                  (1 <strong>+2</strong> ) = 3  | (2 <strong>+2</strong> ) = 4    |   2
                  (3 <strong>-3</strong> ) = 0  | (4 <strong>-3</strong> ) = 1    |  -3
                  (0 <strong>+4</strong> ) = 4  | (1 <strong>+4</strong> ) = 5    |   4
                  (4 <strong>+2</strong> ) = 6  | (5 <strong>+2</strong> ) = 7    |   2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Minimum start value should be positive.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,-2,-3]
<strong>Output:</strong> 5
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `-100 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Prefix Sum
```Rust
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        1 - nums.into_iter().min().unwrap().min(0)
    }
}
```
