# 300. Longest Increasing Subsequence
Given an integer array `nums`, return *the length of the longest **strictly increasing subsequence***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,9,2,5,3,7,101,18]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,1,0,3,2,3]
<strong>Output:</strong> 4
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [7,7,7,7,7,7,7]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= nums.length <= 2500`
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

**Follow up:** Can you come up with an algorithm that runs in `O(n log(n))` time complexity?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut stack = vec![-10001];

        for &num in &nums {
            if num > *stack.last().unwrap() {
                stack.push(num);
            } else if let Err(i) = stack.binary_search(&num) {
                stack[i] = num;
            }
        }

        stack.len() as i32 - 1
    }
}
```
