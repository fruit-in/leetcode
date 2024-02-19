# 2289. 使数组按非递减顺序排列
给你一个下标从 **0** 开始的整数数组 `nums` 。在一步操作中，移除所有满足 `nums[i - 1] > nums[i]` 的 `nums[i]` ，其中 `0 < i < nums.length` 。

重复执行步骤，直到 `nums` 变为 **非递减** 数组，返回所需执行的操作数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,3,4,4,7,3,6,11,8,5,11]
<strong>输出:</strong> 3
<strong>解释:</strong> 执行下述几个步骤：
- 步骤 1 ：[5,3,4,4,7,3,6,11,8,5,11] 变为 [5,4,4,7,6,11,11]
- 步骤 2 ：[5,4,4,7,6,11,11] 变为 [5,4,7,11,11]
- 步骤 3 ：[5,4,7,11,11] 变为 [5,7,11,11]
[5,7,11,11] 是一个非递减数组，因此，返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,5,7,7,13]
<strong>输出:</strong> 0
<strong>解释:</strong> nums 已经是一个非递减数组，因此，返回 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut ret = 0;

        for &num in &nums {
            let mut x = 0;

            while stack.last().unwrap_or(&(i32::MAX, 0)).0 <= num {
                x = x.max(stack.pop().unwrap().1);
            }

            if stack.is_empty() {
                stack.push((num, 0));
            } else {
                stack.push((num, x + 1));
                ret = ret.max(x + 1);
            }
        }

        ret
    }
}
```
