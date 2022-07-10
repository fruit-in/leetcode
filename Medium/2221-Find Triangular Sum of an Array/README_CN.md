# 2221. 数组的三角和
给你一个下标从 **0** 开始的整数数组 `nums` ，其中 `nums[i]` 是 `0` 到 `9` 之间（两者都包含）的一个数字。

`nums` 的 **三角和** 是执行以下操作以后最后剩下元素的值：
1. `nums` 初始包含 `n` 个元素。如果 `n == 1` ，终止 操作。否则，**创建** 一个新的下标从 **0** 开始的长度为 `n - 1` 的整数数组 `newNums` 。
2. 对于满足 `0 <= i < n - 1` 的下标 `i` ，`newNums[i]` **赋值** 为 `(nums[i] + nums[i+1]) % 10` ，`%` 表示取余运算。
3. 将 `newNums` **替换** 数组 `nums` 。
4. 从步骤 1 开始 **重复** 整个过程。

请你返回 `nums` 的三角和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/02/22/ex1drawio.png)
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5]
<strong>输出:</strong> 8
<strong>解释:</strong>
上图展示了得到数组三角和的过程。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5]
<strong>输出:</strong> 5
<strong>解释:</strong>
由于 nums 中只有一个元素，数组的三角和为这个元素自己。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `0 <= nums[i] <= 9`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        while nums.len() > 1 {
            nums = (0..nums.len() - 1)
                .map(|i| (nums[i] + nums[i + 1]) % 10)
                .collect();
        }

        nums[0]
    }
}
```
