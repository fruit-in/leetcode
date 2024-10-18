# 1537. Get the Maximum Score
You are given two **sorted** arrays of distinct integers `nums1` and `nums2`.

A **valid path** is defined as follows:

* Choose array `nums1` or `nums2` to traverse (from index-0).
* Traverse the current array from left to right.
* If you are reading any value that is present in `nums1` and `nums2` you are allowed to change your path to the other array. (Only one repeated value is considered in the valid path).

The **score** is defined as the sum of unique values in a valid path.

Return *the maximum score you can obtain of all possible **valid paths***. Since the answer may be too large, return it modulo <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/07/16/sample_1_1893.png)
<pre>
<strong>Input:</strong> nums1 = [2,4,5,8,10], nums2 = [4,6,8,9]
<strong>Output:</strong> 30
<strong>Explanation:</strong> Valid paths:
[2,4,5,8,10], [2,4,5,8,9], [2,4,6,8,9], [2,4,6,8,10],  (starting from nums1)
[4,6,8,9], [4,5,8,10], [4,5,8,9], [4,6,8,10]    (starting from nums2)
The maximum is obtained with the path in green [2,4,6,8,10].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [1,3,5,7,9], nums2 = [3,5,100]
<strong>Output:</strong> 109
<strong>Explanation:</strong> Maximum sum is obtained with the path [1,3,5,100].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [1,2,3,4,5], nums2 = [6,7,8,9,10]
<strong>Output:</strong> 40
<strong>Explanation:</strong> There are no common elements between nums1 and nums2.
Maximum sum is obtained with the path [6,7,8,9,10].
</pre>

#### Constraints:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[i] <= 10<sup>7</sup></code>
* `nums1` and `nums2` are strictly increasing.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp1 = vec![0; nums1.len()];
        let mut dp2 = vec![0; nums2.len()];
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() || j < nums2.len() {
            if j >= nums2.len() || (i < nums1.len() && nums1[i] < nums2[j]) {
                if i > 0 {
                    dp1[i] = dp1[i - 1];
                }
                dp1[i] += nums1[i] as i64;
                i += 1;
            } else if i >= nums1.len() || (j < nums2.len() && nums1[i] > nums2[j]) {
                if j > 0 {
                    dp2[j] = dp2[j - 1];
                }
                dp2[j] += nums2[j] as i64;
                j += 1;
            } else {
                if i > 0 {
                    dp1[i] = dp1[i - 1];
                }
                if j > 0 {
                    dp1[i] = dp1[i].max(dp2[j - 1]);
                }
                dp1[i] += nums1[i] as i64;
                dp2[j] = dp1[i];
                i += 1;
                j += 1;
            }
        }

        (dp1.pop().unwrap().max(dp2.pop().unwrap()) % 1_000_000_007) as i32
    }
}
```
