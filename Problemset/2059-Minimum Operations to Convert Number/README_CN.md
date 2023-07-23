# 2059. 转化数字的最小运算数
给你一个下标从 **0** 开始的整数数组 `nums` ，该数组由 **互不相同** 的数字组成。另给你两个整数 `start` 和 `goal` 。

整数 `x` 的值最开始设为 `start` ，你打算执行一些运算使 `x` 转化为 `goal` 。你可以对数字 `x` 重复执行下述运算：

如果 `0 <= x <= 1000` ，那么，对于数组中的任一下标 `i`（`0 <= i < nums.length`），可以将 `x` 设为下述任一值：

* `x + nums[i]`
* `x - nums[i]`
* `x ^ nums[i]`（按位异或 XOR）

注意，你可以按任意顺序使用每个 `nums[i]` 任意次。使 `x` 越过 `0 <= x <= 1000` 范围的运算同样可以生效，但该该运算执行后将不能执行其他运算。

返回将 `x = start` 转化为 `goal` 的最小操作数；如果无法完成转化，则返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,4,12], start = 2, goal = 12
<strong>输出:</strong> 2
<strong>解释:</strong>
可以按 2 → 14 → 12 的转化路径进行，只需执行下述 2 次运算：
- 2 + 12 = 14
- 14 - 2 = 12
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,5,7], start = 0, goal = -4
<strong>输出:</strong> 2
<strong>解释:</strong>
可以按 0 → 3 → -4 的转化路径进行，只需执行下述 2 次运算：
- 0 + 3 = 3
- 3 - 7 = -4
注意，最后一步运算使 x 超过范围 0 <= x <= 1000 ，但该运算仍然可以生效。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,8,16], start = 0, goal = 1
<strong>输出:</strong> -1
<strong>解释:</strong>
无法将 0 转化为 1
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>-10<sup>9</sup> <= nums[i], goal <= 10<sup>9</sup></code>
* `0 <= start <= 1000`
* `start != goal`
* `nums` 中的所有整数互不相同

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut unvisited = [true; 1001];
        let mut xs = VecDeque::from([(start, 0)]);

        while let Some((x, count)) = xs.pop_front() {
            for &num in &nums {
                let (a, b, c) = (x + num, x - num, x ^ num);

                if a == goal || b == goal || c == goal {
                    return count + 1;
                }

                if a >= 0 && a <= 1000 && unvisited[a as usize] {
                    unvisited[a as usize] = false;
                    xs.push_back((a, count + 1));
                }
                if b >= 0 && b <= 1000 && unvisited[b as usize] {
                    unvisited[b as usize] = false;
                    xs.push_back((b, count + 1));
                }
                if c >= 0 && c <= 1000 && unvisited[c as usize] {
                    unvisited[c as usize] = false;
                    xs.push_back((c, count + 1));
                }
            }
        }

        -1
    }
}
```
