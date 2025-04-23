# 2321. Maximum Score Of Spliced Array
You are given two **0-indexed** integer arrays `nums1` and `nums2`, both of length `n`.

You can choose two integers `left` and `right` where `0 <= left <= right < n` and **swap** the subarray `nums1[left...right]` with the subarray `nums2[left...right]`.

* For example, if `nums1 = [1,2,3,4,5]` and `nums2 = [11,12,13,14,15]` and you choose `left = 1` and `right = 2`, `nums1` becomes `[1,12,13,4,5]` and `nums2` becomes `[11,2,3,14,15]`.

You may choose to apply the mentioned operation **once** or not do anything.

The **score** of the arrays is the **maximum** of `sum(nums1)` and `sum(nums2)`, where `sum(arr)` is the sum of all the elements in the array `arr`.

Return *the **maximum possible score***.

A **subarray** is a contiguous sequence of elements within an array. `arr[left...right]` denotes the subarray that contains the elements of `nums` between indices `left` and `right` (**inclusive**).

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [60,60,60], nums2 = [10,90,10]
<strong>Output:</strong> 210
<strong>Explanation:</strong> Choosing left = 1 and right = 1, we have nums1 = [60,90,60] and nums2 = [10,60,10].
The score is max(sum(nums1), sum(nums2)) = max(210, 80) = 210.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [20,40,20,70,30], nums2 = [50,20,50,40,20]
<strong>Output:</strong> 220
<strong>Explanation:</strong> Choosing left = 3, right = 4, we have nums1 = [20,40,20,40,20] and nums2 = [50,20,50,70,30].
The score is max(sum(nums1), sum(nums2)) = max(140, 220) = 220.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [7,11,13], nums2 = [1,1,1]
<strong>Output:</strong> 31
<strong>Explanation:</strong> We choose not to swap any subarray.
The score is max(sum(nums1), sum(nums2)) = max(31, 3) = 31.
</pre>

#### Constraints:
* `n == nums1.length == nums2.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut prefix_sum1 = vec![0; n + 1];
        let mut prefix_sum2 = vec![0; n + 1];
        let mut max_diff12 = 0;
        let mut max_diff21 = 0;
        let mut ret = 0;

        for i in 0..n {
            prefix_sum1[i + 1] = prefix_sum1[i] + nums1[i];
            prefix_sum2[i + 1] = prefix_sum2[i] + nums2[i];
        }

        for i in 0..n {
            max_diff12 = max_diff12.max(prefix_sum1[i] - prefix_sum2[i]);
            max_diff21 = max_diff21.max(prefix_sum2[i] - prefix_sum1[i]);
            ret = ret.max(prefix_sum1[n] + prefix_sum2[i + 1] - prefix_sum1[i + 1] + max_diff12);
            ret = ret.max(prefix_sum2[n] + prefix_sum1[i + 1] - prefix_sum2[i + 1] + max_diff21);
        }

        ret
    }
}
```
