# 1035. 不相交的线
在两条独立的水平线上按给定的顺序写下 `nums1` 和 `nums2` 中的整数。

现在，可以绘制一些连接两个数字 `nums1[i]` 和 `nums2[j]` 的直线，这些直线需要同时满足满足：

* `nums1[i] == nums2[j]`
* 且绘制的直线不与任何其他连线（非水平线）相交。

请注意，连线即使在端点也不能相交：每个数字只能属于一条连线。

以这种方法绘制线条，并返回可以绘制的最大连线数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/04/26/142.png)
<pre>
<strong>输入:</strong> nums1 = [1,4,2], nums2 = [1,2,4]
<strong>输出:</strong> 2
<strong>解释:</strong> 可以画出两条不交叉的线，如上图所示。
但无法画出第三条不相交的直线，因为从 nums1[1]=4 到 nums2[2]=4 的直线将与从 nums1[2]=2 到 nums2[1]=2 的直线相交。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [2,5,1,2,5], nums2 = [10,5,2,1,5,2]
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [1,3,7,1,7,5], nums2 = [1,9,2,5,1]
<strong>输出:</strong> 2
</pre>

#### 提示:
* `1 <= nums1.length, nums2.length <= 500`
* `1 <= nums1[i], nums2[j] <= 2000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if nums1[i] == nums2[j] {
                    if i > 0 && j > 0 {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                    dp[i][j] += 1;
                }
                if i > 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1]);
                }
            }
        }

        dp[m - 1][n - 1]
    }
}
```
