# 456. 132 模式
给你一个整数数组 `nums` ，数组中共有 `n` 个整数。**132 模式的子序列** 由三个整数 `nums[i]`、`nums[j]` 和 `nums[k]` 组成，并同时满足：`i < j < k` 和 `nums[i] < nums[k] < nums[j]` 。

如果 `nums` 中存在 **132 模式的子序列** ，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> false
<strong>解释:</strong> 序列中不存在 132 模式的子序列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,1,4,2]
<strong>输出:</strong> true
<strong>解释:</strong> 序列中有 1 个 132 模式的子序列： [1, 4, 2] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [-1,3,2,0]
<strong>输出:</strong> true
<strong>解释:</strong> 序列中有 3 个 132 模式的的子序列：[-1, 3, 2]、[-1, 3, 0] 和 [-1, 2, 0] 。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 2 * 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut min_num = i32::MAX;

        for k in 0..nums.len() {
            while let Some(&(numsi, numsj)) = stack.last() {
                if numsi < nums[k] && nums[k] < numsj {
                    return true;
                } else if nums[k] >= numsj {
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((min_num, nums[k]));
            min_num = min_num.min(nums[k]);
        }

        false
    }
}
```
