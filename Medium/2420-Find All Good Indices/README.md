# 2420. Find All Good Indices
You are given a **0-indexed** integer array `nums` of size `n` and a positive integer `k`.

We call an index `i` in the range `k <= i < n - k` **good** if the following conditions are satisfied:

* The `k` elements that are just **before** the index `i` are in **non-increasing** order.
* The `k` elements that are just **after** the index `i` are in **non-decreasing** order.

Return *an array of all good indices sorted in **increasing** order*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,1,1,3,4,1], k = 2
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong> There are two good indices in the array:
- Index 2. The subarray [2,1] is in non-increasing order, and the subarray [1,3] is in non-decreasing order.
- Index 3. The subarray [1,1] is in non-increasing order, and the subarray [3,4] is in non-decreasing order.
Note that the index 4 is not good because [4,1] is not non-decreasing.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,1,1,2], k = 2
<strong>Output:</strong> []
<strong>Explanation:</strong> There are no good indices in this array.
</pre>

#### Constraints:
* `n == nums.length`
* <code>3 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* `1 <= k <= n / 2`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        let mut count = (1, 1);

        for i in 1..n {
            prefix[i] = count.0;
            suffix[n - 1 - i] = count.1;

            if nums[i] <= nums[i - 1] {
                count.0 += 1;
            } else {
                count.0 = 1;
            }
            if nums[n - 1 - i] <= nums[n - i] {
                count.1 += 1;
            } else {
                count.1 = 1;
            }
        }

        (k..n - k)
            .filter(|&i| prefix[i].min(suffix[i]) >= k)
            .map(|i| i as i32)
            .collect()
    }
}
```
