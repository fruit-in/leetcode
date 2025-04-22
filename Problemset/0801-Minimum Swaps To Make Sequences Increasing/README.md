# 801. Minimum Swaps To Make Sequences Increasing
You are given two integer arrays of the same length `nums1` and `nums2`. In one operation, you are allowed to swap `nums1[i]` with `nums2[i]`.

* For example, if `nums1 = [1,2,3,8]`, and `nums2 = [5,6,7,4]`, you can swap the element at `i = 3` to obtain `nums1 = [1,2,3,4]` and `nums2 = [5,6,7,8]`.

Return *the minimum number of needed operations to make* `nums1` *and* `nums2` ***strictly increasing***. The test cases are generated so that the given input always makes it possible.

An array `arr` is **strictly increasing** if and only if `arr[0] < arr[1] < arr[2] < ... < arr[arr.length - 1]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,3,5,4], nums2 = [1,2,3,7]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Swap nums1[3] and nums2[3]. Then the sequences are:
nums1 = [1, 3, 5, 7] and nums2 = [1, 2, 3, 4]
which are both strictly increasing.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [0,3,5,8,9], nums2 = [2,1,4,6,9]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>2 <= nums1.length <= 10<sup>5</sup></code>
* `nums2.length == nums1.length`
* <code>0 <= nums1[i], nums2[i] <= 2 * 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = [[0, 1], [i32::MAX; 2]];

        for i in 1..nums1.len() {
            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                dp[1][0] = dp[0][0];
                dp[1][1] = dp[0][1] + 1;
            }
            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                dp[1][0] = dp[1][0].min(dp[0][1]);
                dp[1][1] = dp[1][1].min(dp[0][0] + 1);
            }

            dp = [dp[1], [i32::MAX; 2]];
        }

        dp[0][0].min(dp[0][1])
    }
}
```
