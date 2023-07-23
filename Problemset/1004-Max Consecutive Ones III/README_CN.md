# 1004. 最大连续1的个数 III
给定一个二进制数组 `nums` 和一个整数 `k`，如果可以翻转最多 `k` 个 `0` ，则返回 *数组中连续 `1` 的最大个数* 。
Given a binary array `nums` and an integer `k`, return *the maximum number of consecutive* `1`*'s in the array if you can flip at most* `k` `0`'s.

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
<strong>输出:</strong> 6
<strong>解释:</strong> [1,1,1,0,0,1,1,1,1,1,1]
粗体数字从 0 翻转到 1，最长的子数组长度为 6。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
<strong>输出:</strong> 10
<strong>解释:</strong> [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
粗体数字从 0 翻转到 1，最长的子数组长度为 10。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums[i]` 不是 `0` 就是 `1`
* `0 <= k <= nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut count0 = 0;
        let mut ret = 0;

        while r < nums.len() {
            if nums[r] == 0 {
                count0 += 1;
                while count0 > k {
                    if nums[l] == 0 {
                        count0 -= 1;
                    }
                    l += 1;
                }
            }

            r += 1;
            ret = ret.max(r - l);
        }

        ret as i32
    }
}
```
