# 1929. Concatenation of Array
Given an integer array `nums` of length `n`, you want to create an array `ans` of length `2n` where `ans[i] == nums[i]` and `ans[i + n] == nums[i]` for `0 <= i < n` (**0-indexed**).

Specifically, `ans` is the **concatenation** of two `nums` arrays.

Return *the array* `ans`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,1]
<strong>Output:</strong> [1,2,1,1,2,1]
<strong>Explanation:</strong> The array ans is formed as follows:
- ans = [nums[0],nums[1],nums[2],nums[0],nums[1],nums[2]]
- ans = [1,2,1,1,2,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,3,2,1]
<strong>Output:</strong> [1,3,2,1,1,3,2,1]
<strong>Explanation:</strong> The array ans is formed as follows:
- ans = [nums[0],nums[1],nums[2],nums[3],nums[0],nums[1],nums[2],nums[3]]
- ans = [1,3,2,1,1,3,2,1]
</pre>

#### Constraints:
* `n == nums.length`
* `1 <= n <= 1000`
* `1 <= nums[i] <= 1000`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def getConcatenation(self, nums: List[int]) -> List[int]:
        return nums + nums
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.clone().into_iter().chain(nums.into_iter()).collect()
    }
}
```
