# 2425. 所有数对的异或和
给你两个下标从 **0** 开始的数组 `nums1` 和 `nums2` ，两个数组都只包含非负整数。请你求出另外一个数组 `nums3` ，包含 `nums1` 和 `nums2` 中 **所有数对** 的异或和（`nums1` 中每个整数都跟 `nums2` 中每个整数 **恰好** 匹配一次）。

请你返回 `nums3` 中所有整数的 **异或和** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [2,1,3], nums2 = [10,2,5,0]
<strong>输出:</strong> 13
<strong>解释:</strong>
一个可能的 nums3 数组是 [8,0,7,2,11,3,4,1,9,1,6,3] 。
所有这些数字的异或和是 13 ，所以我们返回 13 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,2], nums2 = [3,4]
<strong>输出:</strong> 0
<strong>解释:</strong>
所有数对异或和的结果分别为 nums1[0] ^ nums2[0] ，nums1[0] ^ nums2[1] ，nums1[1] ^ nums2[0] 和 nums1[1] ^ nums2[1] 。
所以，一个可能的 nums3 数组是 [2,5,1,6] 。
2 ^ 5 ^ 1 ^ 6 = 0 ，所以我们返回 0 。
</pre>

#### 提示:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>0 <= nums1[i], nums2[j] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        match (nums1.len() % 2, nums2.len() % 2) {
            (0, 0) => 0,
            (0, 1) => nums1.iter().fold(0, |acc, x| acc ^ x),
            (1, 0) => nums2.iter().fold(0, |acc, x| acc ^ x),
            _ => nums1.iter().chain(nums2.iter()).fold(0, |acc, x| acc ^ x),
        }
    }
}
```
