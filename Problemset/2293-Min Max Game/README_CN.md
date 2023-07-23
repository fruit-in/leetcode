# 2293. 极大极小游戏
给你一个下标从 **0** 开始的整数数组 `nums` ，其长度是 `2` 的幂。

对 `nums` 执行下述算法：
1. 设 `n` 等于 `nums` 的长度，如果 `n == 1` ，**终止** 算法过程。否则，**创建** 一个新的整数数组 `newNums` ，新数组长度为 `n / 2` ，下标从 **0** 开始。
2. 对于满足 `0 <= i < n / 2` 的每个 **偶数** 下标 `i` ，将 `newNums[i]` **赋值** 为 `min(nums[2 * i], nums[2 * i + 1])` 。
3. 对于满足 `0 <= i < n / 2` 的每个 **奇数** 下标 `i` ，将 `newNums[i]` **赋值** 为 `max(nums[2 * i], nums[2 * i + 1])` 。
4. 用 `newNums` 替换 `nums` 。
5. 从步骤 1 开始 **重复** 整个过程。

执行算法后，返回 `nums` 中剩下的那个数字。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/04/13/example1drawio-1.png)
<pre>
<strong>输入:</strong> nums = [1,3,5,2,4,8,2,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 重复执行算法会得到下述数组。
第一轮：nums = [1,5,4,2]
第二轮：nums = [1,4]
第三轮：nums = [1]
1 是最后剩下的那个数字，返回 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3]
<strong>输出:</strong> 3
<strong>解释:</strong> 3 就是最后剩下的数字，返回 3 。
</pre>

#### 提示:
* `1 <= nums.length <= 1024`
* <code>1 <= nums[i] <= 10<code>9</sup></code>
* `nums.length` 是 `2` 的幂

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut n = nums.len();

        while n > 1 {
            n /= 2;
            nums = (0..n)
                .map(|i| match i % 2 {
                    0 => nums[2 * i].min(nums[2 * i + 1]),
                    _ => nums[2 * i].max(nums[2 * i + 1]),
                })
                .collect();
        }

        nums[0]
    }
}
```
