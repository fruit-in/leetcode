# 1458. Max Dot Product of Two Subsequences
Given two arrays `nums1` and `nums2`.

Return the maximum dot product between **non-empty** subsequences of nums1 and nums2 with the same length.

A subsequence of a array is a new array which is formed from the original array by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, `[2,3,5]` is a subsequence of `[1,2,3,4,5]` while `[1,5,3]` is not).

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [2,1,-2,5], nums2 = [3,0,-6]
<strong>Output:</strong> 18
<strong>Explanation:</strong> Take subsequence [2,-2] from nums1 and subsequence [3,-6] from nums2.
Their dot product is (2*3 + (-2)*(-6)) = 18.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [3,-2], nums2 = [2,-6,7]
<strong>Output:</strong> 21
<strong>Explanation:</strong> Take subsequence [3] from nums1 and subsequence [7] from nums2.
Their dot product is (3*7) = 21.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [-1,-1], nums2 = [1,1]
<strong>Output:</strong> -1
<strong>Explanation:</strong> Take subsequence [-1] from nums1 and subsequence [1] from nums2.
Their dot product is -1.
</pre>

#### Constraints:
* `1 <= nums1.length, nums2.length <= 500`
* `-1000 <= nums1[i], nums2[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut ret = i32::MIN;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                ret = ret.max(nums1[i] * nums2[j]);
                dp[i + 1][j + 1] = dp[i + 1][j + 1]
                    .max(dp[i][j + 1])
                    .max(dp[i + 1][j])
                    .max(dp[i][j] + nums1[i] * nums2[j]);
            }
        }

        if ret >= 0 {
            ret = ret.max(dp[nums1.len()][nums2.len()]);
        }

        ret
    }
}
```
