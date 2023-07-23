# 739. 每日温度
请根据每日 `气温` 列表，重新生成一个列表。对应位置的输出为：要想观测到更高的气温，至少需要等待的天数。如果气温在这之后都不会升高，请在该位置用 `0` 来代替。

例如，给定一个列表 `temperatures = [73, 74, 75, 71, 69, 72, 76, 73]`，你的输出应该是 `[1, 1, 4, 2, 1, 1, 0, 0]`。

**提示:** `气温` 列表长度的范围是 `[1, 30000]`。每个气温的值的均为华氏度，都是在 `[30, 100]` 范围内的整数。

## 题解 (Rust)

### 1. 栈
```Rust
impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![0; t.len()];

        for i in (0..t.len()).rev() {
            while stack.last().unwrap_or(&(101, 0)).0 <= t[i] {
                stack.pop();
            }
            ret[i] = (stack.last().unwrap_or(&(0, i)).1 - i) as i32;
            stack.push((t[i], i));
        }

        ret
    }
}
```
