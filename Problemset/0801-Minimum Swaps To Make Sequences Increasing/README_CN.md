# 801. 使序列递增的最小交换次数
我们有两个长度相等且不为空的整型数组 `nums1` 和 `nums2` 。在一次操作中，我们可以交换 `nums1[i]` 和 `nums2[i]`的元素。

* 例如，如果 `nums1 = [1,2,3,8]` ， `nums2 =[5,6,7,4]` ，你可以交换 `i = 3` 处的元素，得到 `nums1 =[1,2,3,4]` 和 `nums2 =[5,6,7,8]` 。

返回 *使 `nums1` 和 `nums2` **严格递增** 所需操作的最小次数* 。

数组 `arr` **严格递增** 且  `arr[0] < arr[1] < arr[2] < ... < arr[arr.length - 1]` 。

**注意：**
* 用例保证可以实现操作。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,3,5,4], nums2 = [1,2,3,7]
<strong>输出:</strong> 1
<strong>解释:</strong>
交换 A[3] 和 B[3] 后，两个数组如下:
A = [1, 3, 5, 7] ， B = [1, 2, 3, 4]
两个数组均为严格递增的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [0,3,5,8,9], nums2 = [2,1,4,6,9]
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>2 <= nums1.length <= 10<sup>5</sup></code>
* `nums2.length == nums1.length`
* <code>0 <= nums1[i], nums2[i] <= 2 * 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
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
