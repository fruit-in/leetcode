# 397. 整数替换
给定一个正整数 `n` ，你可以做如下操作：
1. 如果 `n` 是偶数，则用 `n / 2`替换 `n` 。
2. 如果 `n` 是奇数，则可以用 `n + 1`或`n - 1`替换 `n` 。

`n` 变为 `1` 所需的最小替换次数是多少？

#### 示例 1:
<pre>
<strong>输入:</strong> n = 8
<strong>输出:</strong> 3
<strong>解释:</strong> 8 -> 4 -> 2 -> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 7
<strong>输出:</strong> 4
<strong>解释:</strong> 7 -> 8 -> 4 -> 2 -> 1
or 7 -> 6 -> 3 -> 2 -> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> 2
</pre>

#### 提示:
* <code>1 <= n <= 2<sup>31</sup> - 1</code>

## 题解 (Rust)

### 1. 广度优先搜索
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut nums = vec![(n as i64, 0)].into_iter().collect::<VecDeque<_>>();
        let mut seen = vec![n as i64].into_iter().collect::<HashSet<_>>();

        while let Some((x, y)) = nums.pop_front() {
            if x == 1 {
                return y;
            }
            if x % 2 == 0 && !seen.contains(&(x / 2)) {
                nums.push_back((x / 2, y + 1));
                seen.insert(x / 2);
            } else if x % 2 == 1 {
                if !seen.contains(&(x + 1)) {
                    nums.push_back((x + 1, y + 1));
                    seen.insert(x + 1);
                }
                if !seen.contains(&(x - 1)) {
                    nums.push_back((x - 1, y + 1));
                    seen.insert(x - 1);
                }
            }
        }

        0
    }
}
```
