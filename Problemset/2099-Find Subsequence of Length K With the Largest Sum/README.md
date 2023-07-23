# 2099. Find Subsequence of Length K With the Largest Sum
You are given an integer array `nums` and an integer `k`. You want to find a **subsequence** of `nums` of length `k` that has the **largest** sum.

Return ***any** such subsequence as an integer array of length* `k`.

A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,3,3], k = 2
<strong>Output:</strong> [3,3]
<strong>Explanation:</strong>
The subsequence has the largest sum of 3 + 3 = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-1,-2,3,4], k = 3
<strong>Output:</strong> [-1,3,4]
<strong>Explanation:</strong>
The subsequence has the largest sum of -1 + 3 + 4 = 6.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,4,3,3], k = 2
<strong>Output:</strong> [3,4]
<strong>Explanation:</strong>
The subsequence has the largest sum of 3 + 4 = 7.
Another possible subsequence is [4, 3].
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>
* `1 <= k <= nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        nums.truncate(k as usize);
        nums.sort_unstable();

        nums.into_iter().map(|(_, num)| num).collect()
    }
}
```
