# 1218. 最长定差子序列
给你一个整数数组 `arr` 和一个整数 `difference`，请你找出并返回 `arr` 中最长等差子序列的长度，该子序列中相邻元素之间的差等于 `difference` 。

**子序列** 是指在不改变其余元素顺序的情况下，通过删除一些元素或不删除任何元素而从 `arr` 派生出来的序列。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,3,4], difference = 1
<strong>输出:</strong> 4
<strong>解释:</strong> 最长的等差子序列是 [1,2,3,4]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,3,5,7], difference = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 最长的等差子序列是任意单个元素。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,5,7,8,5,3,4,2,1], difference = -2
<strong>输出:</strong> 4
<strong>解释:</strong> 最长的等差子序列是 [7,5,3,1]。
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= arr[i], difference <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut longest_length = HashMap::new();

        for &num in &arr {
            longest_length.insert(
                num,
                *longest_length.get(&(num - difference)).unwrap_or(&0) + 1,
            );
        }

        *longest_length.values().max().unwrap()
    }
}
```
