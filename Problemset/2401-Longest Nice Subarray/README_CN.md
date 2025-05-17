# 2401. 最长优雅子数组
给你一个由 **正** 整数组成的数组 `nums` 。

如果 `nums` 的子数组中位于 **不同** 位置的每对元素按位 **与（AND）**运算的结果等于 `0` ，则称该子数组为 **优雅** 子数组。

返回 **最长** 的优雅子数组的长度。

**子数组** 是数组中的一个 **连续** 部分。

**注意：**长度为 `1` 的子数组始终视作优雅子数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,8,48,10]
<strong>输出:</strong> 3
<strong>解释:</strong> 最长的优雅子数组是 [3,8,48] 。子数组满足题目条件：
- 3 AND 8 = 0
- 3 AND 48 = 0
- 8 AND 48 = 0
可以证明不存在更长的优雅子数组，所以返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,1,5,11,13]
<strong>输出:</strong> 1
<strong>解释:</strong> 最长的优雅子数组长度为 1 ，任何长度为 1 的子数组都满足题目条件。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut mask = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            while mask & nums[j] != 0 {
                mask ^= nums[i];
                i += 1;
            }
            ret = ret.max(j - i + 1);
            mask |= nums[j];
        }

        ret as i32
    }
}
```
