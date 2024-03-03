# 1696. 跳跃游戏 VI
给你一个下标从 **0** 开始的整数数组 `nums` 和一个整数 `k` 。

一开始你在下标 `0` 处。每一步，你最多可以往前跳 `k` 步，但你不能跳出数组的边界。也就是说，你可以从下标 `i` 跳到 `[i + 1， min(n - 1, i + k)]` **包含** 两个端点的任意位置。

你的目标是到达数组最后一个位置（下标为 `n - 1` ），你的 **得分** 为经过的所有数字之和。

请你返回你能得到的 **最大得分** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,-1,-2,4,-7,3], k = 2
<strong>输出:</strong> 7
<strong>解释:</strong> 你可以选择子序列 [1,-1,4,3] （上面加粗的数字），和为 7 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [10,-5,-2,4,0,3], k = 3
<strong>输出:</strong> 17
<strong>解释:</strong> 你可以选择子序列 [10,4,3] （上面加粗数字），和为 17 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,-5,-20,4,-1,3,-6,-3], k = 2
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= nums.length, k <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut deque = VecDeque::from([(0, nums[0])]);

        for i in 1..nums.len() {
            if i - deque[0].0 > k {
                deque.pop_front();
            }

            let x = nums[i] + deque[0].1;

            while deque.back().unwrap_or(&(0, i32::MAX)).1 <= x {
                deque.pop_back();
            }

            deque.push_back((i, x));
        }

        deque.pop_back().unwrap().1
    }
}
```
