# 2389. Longest Subsequence With Limited Sum
You are given an integer array `nums` of length `n`, and an integer array `queries` of length `m`.

Return *an array* `answer` *of length* `m` *where* `answer[i]` *is the **maximum** size of a **subsequence** that you can take from* `nums` *such that the **sum** of its elements is less than or equal to* `queries[i]`.

A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,5,2,1], queries = [3,10,21]
<strong>Output:</strong> [2,3,4]
<strong>Explanation:</strong> We answer the queries as follows:
- The subsequence [2,1] has a sum less than or equal to 3. It can be proven that 2 is the maximum size of such a subsequence, so answer[0] = 2.
- The subsequence [4,5,1] has a sum less than or equal to 10. It can be proven that 3 is the maximum size of such a subsequence, so answer[1] = 3.
- The subsequence [4,5,2,1] has a sum less than or equal to 21. It can be proven that 4 is the maximum size of such a subsequence, so answer[2] = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,4,5], queries = [1]
<strong>Output:</strong> [0]
<strong>Explanation:</strong> The empty subsequence is the only subsequence that has a sum less than or equal to 1, so answer[0] = 0.
</pre>

#### Constraints:
* `n == nums.length`
* `m == queries.length`
* `1 <= n, m <= 1000`
* <code>1 <= nums[i], queries[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut answer = vec![0; queries.len()];

        nums.sort_unstable();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        for i in 0..queries.len() {
            answer[i] = match nums.binary_search(&queries[i]) {
                Ok(j) => j as i32 + 1,
                Err(j) => j as i32,
            };
        }

        answer
    }
}
```
