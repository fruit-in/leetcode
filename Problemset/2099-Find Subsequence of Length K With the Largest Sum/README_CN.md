# 2099. 找到和最大的长度为 K 的子序列
给你一个整数数组 `nums` 和一个整数 `k` 。你需要找到 `nums` 中长度为 `k` 的 **子序列** ，且这个子序列的 **和最大** 。

请你返回 **任意** 一个长度为 `k` 的整数子序列。

**子序列** 定义为从一个数组里删除一些元素后，不改变剩下元素的顺序得到的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,3,3], k = 2
<strong>输出:</strong> [3,3]
<strong>解释:</strong>
子序列有最大和：3 + 3 = 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-1,-2,3,4], k = 3
<strong>输出:</strong> [-1,3,4]
<strong>解释:</strong>
子序列有最大和：-1 + 3 + 4 = 6 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,4,3,3], k = 2
<strong>输出:</strong> [3,4]
<strong>解释:</strong>
子序列有最大和：3 + 4 = 7 。
另一个可行的子序列为 [4, 3] 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>
* `1 <= k <= nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        nums.truncate(k as usize);
        nums.sort_unstable();

        nums.into_iter().map(|(_, num)| num).collect()
    }
}
```
