# 1537. 最大得分
你有两个 **有序** 且数组内元素互不相同的数组 `nums1` 和 `nums2` 。

一条 **合法路径** 定义如下：

* 选择数组 `nums1` 或者 `nums2` 开始遍历（从下标 0 处开始）。
* 从左到右遍历当前数组。
* 如果你遇到了 `nums1` 和 `nums2` 中都存在的值，那么你可以切换路径到另一个数组对应数字处继续遍历（但在合法路径中重复数字只会被统计一次）。

**得分** 定义为合法路径中不同数字的和。

请你返回 *所有可能 **合法路径** 中的最大得分。*由于答案可能很大，请你将它对 10^9 + 7 取余后返回。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/07/16/sample_1_1893.png)
<pre>
<strong>输入:</strong> nums1 = [2,4,5,8,10], nums2 = [4,6,8,9]
<strong>输出:</strong> 30
<strong>解释:</strong> 合法路径包括：
[2,4,5,8,10], [2,4,5,8,9], [2,4,6,8,9], [2,4,6,8,10],（从 nums1 开始遍历）
[4,6,8,9], [4,5,8,10], [4,5,8,9], [4,6,8,10]  （从 nums2 开始遍历）
最大得分为上图中的绿色路径 [2,4,6,8,10] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,3,5,7,9], nums2 = [3,5,100]
<strong>输出:</strong> 109
<strong>解释:</strong> 最大得分由路径 [1,3,5,100] 得到。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [1,2,3,4,5], nums2 = [6,7,8,9,10]
<strong>输出:</strong> 40
<strong>解释:</strong> nums1 和 nums2 之间无相同数字。
最大得分由路径[6,7,8,9,10]得到。
</pre>

#### 提示:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[i] <= 10<sup>7</sup></code>
* `nums1` 和 `nums2` 都是严格递增的数组。

## 题解 (Rust)

### 1. 题解
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
