# 2221. Find Triangular Sum of an Array
You are given a **0-indexed** integer array `nums`, where `nums[i]` is a digit between `0` and `9` (**inclusive**).

The **triangular sum** of nums is the value of the only element present in nums after the following process terminates:
1. Let `nums` comprise of `n` elements. If `n == 1`, **end** the process. Otherwise, **create** a new **0-indexed** integer array `newNums` of length `n - 1`.
2. For each index `i`, where `0 <= i < n - 1`, **assign** the value of `newNums[i]` as `(nums[i] + nums[i+1]) % 10`, where `%` denotes modulo operator.
3. **Replace** the array `nums` with `newNums`.
4. **Repeat** the entire process starting from step 1.

Return *the triangular sum of* `nums`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/22/ex1drawio.png)
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> 8
<strong>Explanation:</strong>
The above diagram depicts the process from which we obtain the triangular sum of the array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5]
<strong>Output:</strong> 5
<strong>Explanation:</strong>
Since there is only one element in nums, the triangular sum is the value of that element itself.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `0 <= nums[i] <= 9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        while nums.len() > 1 {
            nums = (0..nums.len() - 1)
                .map(|i| (nums[i] + nums[i + 1]) % 10)
                .collect();
        }

        nums[0]
    }
}
```
