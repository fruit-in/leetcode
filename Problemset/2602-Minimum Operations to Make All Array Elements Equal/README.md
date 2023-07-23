# 2602. Minimum Operations to Make All Array Elements Equal
You are given an array `nums` consisting of positive integers.

You are also given an integer array `queries` of size `m`. For the <code>i<sup>th</sup></code> query, you want to make all of the elements of `nums` equal to `queries[i]`. You can perform the following operation on the array **any** number of times:

* **Increase** or **decrease** an element of the array by `1`.

Return *an array* `answer` *of size* `m` *where* `answer[i]` *is the **minimum** number of operations to make all elements of* `nums` *equal to* `queries[i]`.

**Note** that after each query the array is reset to its original state.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,1,6,8], queries = [1,5]
<strong>Output:</strong> [14,10]
<strong>Explanation:</strong> For the first query we can do the following operations:
- Decrease nums[0] 2 times, so that nums = [1,1,6,8].
- Decrease nums[2] 5 times, so that nums = [1,1,1,8].
- Decrease nums[3] 7 times, so that nums = [1,1,1,1].
So the total number of operations for the first query is 2 + 5 + 7 = 14.
For the second query we can do the following operations:
- Increase nums[0] 2 times, so that nums = [5,1,6,8].
- Increase nums[1] 4 times, so that nums = [5,5,6,8].
- Decrease nums[2] 1 time, so that nums = [5,5,5,8].
- Decrease nums[3] 3 times, so that nums = [5,5,5,5].
So the total number of operations for the second query is 2 + 4 + 1 + 3 = 10.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,9,6,3], queries = [10]
<strong>Output:</strong> [20]
<strong>Explanation:</strong> We can increase each value in the array to 10. The total number of operations will be 8 + 1 + 4 + 7 = 20.
</pre>

#### Constraints:
* `n == nums.length`
* `m == queries.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* <code>1 <= nums[i], queries[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let (n, m) = (nums.len(), queries.len());
        let mut nums = nums;
        let mut prefix_sum = vec![0; n];
        let mut answer = vec![0; m];

        nums.sort_unstable();
        prefix_sum[0] = nums[0] as i64;

        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
        }

        for i in 0..queries.len() {
            let (j, x) = match nums.binary_search(&queries[i]) {
                Ok(0) | Err(0) => (0, 0),
                Ok(k) | Err(k) => (k as i64, prefix_sum[k - 1]),
            };

            answer[i] = (2 * j - n as i64) * queries[i] as i64 + prefix_sum[n - 1] - 2 * x;
        }

        answer
    }
}
```
