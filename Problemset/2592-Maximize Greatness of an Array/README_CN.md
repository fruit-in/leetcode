# 2592. 最大化数组的伟大值
给你一个下标从 0 开始的整数数组 `nums` 。你需要将 `nums` 重新排列成一个新的数组 `perm` 。

定义 `nums` 的 **伟大值** 为满足 `0 <= i < nums.length` 且 `perm[i] > nums[i]` 的下标数目。

请你返回重新排列 `nums` 后的 **最大** 伟大值。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,5,2,1,3,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 一个最优安排方案为 perm = [2,5,1,3,3,1,1] 。
在下标为 0, 1, 3 和 4 处，都有 perm[i] > nums[i] 。因此我们返回 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> 3
<strong>解释:</strong> 最优排列为 [2,3,4,1] 。
在下标为 0, 1 和 2 处，都有 perm[i] > nums[i] 。因此我们返回 3 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut ret = 0;

        nums.sort_unstable();

        for j in 0..nums.len() {
            while i < nums.len() && nums[i] <= nums[j] {
                i += 1;
            }

            if i >= nums.len() {
                break;
            } else {
                i += 1;
                ret += 1;
            }
        }

        ret
    }
}
```
