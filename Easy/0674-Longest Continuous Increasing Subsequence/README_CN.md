# 674. 最长连续递增序列
给定一个未经排序的整数数组，找到最长且**连续**的递增序列。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,3,5,4,7]
<strong>输出:</strong> 3
<strong>解释:</strong> 最长连续递增序列是 [1,3,5], 长度为3。
尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为5和7在原数组里被4隔开。 
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [2,2,2,2,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 最长连续递增序列是 [2], 长度为1。
</pre>

**注意:** 数组长度不会超过10000。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut max_len = 0;

        for j in 1..=nums.len() {
            if j == nums.len() || nums[j] <= nums[j - 1] {
                max_len = max_len.max(j - i);
                i = j;
            }
        }

        max_len as i32
    }
}
```
