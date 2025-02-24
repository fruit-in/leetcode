# 718. 最长重复子数组
给两个整数数组 `nums1` 和 `nums2` ，返回 *两个数组中 **公共的** 、长度最长的子数组的长度* 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
<strong>输出:</strong> 3
<strong>解释:</strong> 长度最长的公共子数组是 [3,2,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [0,0,0,0,0], nums2 = [0,0,0,0,0]
<strong>输出:</strong> 5
</pre>

#### 提示:
* `1 <= nums1.length, nums2.length <= 1000`
* `0 <= nums1[i], nums2[i] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut ret = 0;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] == nums2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                    ret = ret.max(dp[i + 1][j + 1]);
                }
            }
        }

        ret
    }
}
```
