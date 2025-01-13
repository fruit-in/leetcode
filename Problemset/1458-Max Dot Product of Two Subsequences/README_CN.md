# 1458. 两个子序列的最大点积
给你两个数组 `nums1` 和 `nums2` 。

请你返回 `nums1` 和 `nums2` 中两个长度相同的 **非空** 子序列的最大点积。

数组的非空子序列是通过删除原数组中某些元素（可能一个也不删除）后剩余数字组成的序列，但不能改变数字间相对顺序。比方说，`[2,3,5]` 是 `[1,2,3,4,5]` 的一个子序列而 `[1,5,3]` 不是。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [2,1,-2,5], nums2 = [3,0,-6]
<strong>输出:</strong> 18
<strong>解释:</strong> 从 nums1 中得到子序列 [2,-2] ，从 nums2 中得到子序列 [3,-6] 。
它们的点积为 (2*3 + (-2)*(-6)) = 18 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [3,-2], nums2 = [2,-6,7]
<strong>输出:</strong> 21
<strong>解释:</strong> 从 nums1 中得到子序列 [3] ，从 nums2 中得到子序列 [7] 。
它们的点积为 (3*7) = 21 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [-1,-1], nums2 = [1,1]
<strong>输出:</strong> -1
<strong>解释:</strong> 从 nums1 中得到子序列 [-1] ，从 nums2 中得到子序列 [1] 。
它们的点积为 -1 。
</pre>

#### 提示:
* `1 <= nums1.length, nums2.length <= 500`
* `-1000 <= nums1[i], nums2[i] <= 1000`

#### 点积:
定义 <code>a = [a<sub>1</sub>, a<sub>2</sub>,…, a<sub>n</sub>]</code> 和 <code>b = [b<sub>1</sub>, b<sub>2</sub>,…, b<sub>n</sub>]</code> 的点积为：

![](https://pic.leetcode-cn.com/1666164309-PBJMQp-image.png)

这里的 **Σ** 指示总和符号。

## 题解 (Rust)

### 1. 题解
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
