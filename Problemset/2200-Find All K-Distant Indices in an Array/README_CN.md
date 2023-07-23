# 2200. 找出数组中的所有 K 近邻下标
给你一个下标从 **0** 开始的整数数组 `nums` 和两个整数 `key` 和 `k` 。**K 近邻下标** 是 `nums` 中的一个下标 `i` ，并满足至少存在一个下标 `j` 使得 `|i - j| <= k` 且 `nums[j] == key` 。

以列表形式返回按 **递增顺序** 排序的所有 K 近邻下标。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,4,9,1,3,9,5], key = 9, k = 1
<strong>输出:</strong> [1,2,3,4,5,6]
<strong>解释:</strong> 因此，nums[2] == key 且 nums[5] == key 。
- 对下标 0 ，|0 - 2| > k 且 |0 - 5| > k ，所以不存在 j 使得 |0 - j| <= k 且 nums[j] == key 。所以 0 不是一个 K 近邻下标。
- 对下标 1 ，|1 - 2| <= k 且 nums[2] == key ，所以 1 是一个 K 近邻下标。
- 对下标 2 ，|2 - 2| <= k 且 nums[2] == key ，所以 2 是一个 K 近邻下标。
- 对下标 3 ，|3 - 2| <= k 且 nums[2] == key ，所以 3 是一个 K 近邻下标。
- 对下标 4 ，|4 - 5| <= k 且 nums[5] == key ，所以 4 是一个 K 近邻下标。
- 对下标 5 ，|5 - 5| <= k 且 nums[5] == key ，所以 5 是一个 K 近邻下标。
- 对下标 6 ，|6 - 5| <= k 且 nums[5] == key ，所以 6 是一个 K 近邻下标。
因此，按递增顺序返回 [1,2,3,4,5,6] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2,2,2], key = 2, k = 2
<strong>输出:</strong> [0,1,2,3,4]
<strong>解释:</strong> 对 nums 的所有下标 i ，总存在某个下标 j 使得 |i - j| <= k 且 nums[j] == key ，所以每个下标都是一个 K 近邻下标。
因此，返回 [0,1,2,3,4] 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`
* `key` 是数组 `nums` 中的一个整数
* `1 <= k <= nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut ret = vec![];

        for j in 0..nums.len() {
            if nums[j] == key {
                for i in (j as i32 - k).max(0)..(j as i32 + k + 1).min(nums.len() as i32) {
                    if i > *ret.last().unwrap_or(&-1) {
                        ret.push(i);
                    }
                }
            }
        }

        ret
    }
}
```
