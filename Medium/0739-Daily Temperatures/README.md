# 739. Daily Temperatures
Given a list of daily temperatures `T`, return a list such that, for each day in the input, tells you how many days you would have to wait until a warmer temperature. If there is no future day for which this is possible, put `0` instead.

For example, given the list of temperatures `T = [73, 74, 75, 71, 69, 72, 76, 73]`, your output should be `[1, 1, 4, 2, 1, 1, 0, 0]`.

**Note:** The length of `temperatures` will be in the range `[1, 30000]`. Each temperature will be an integer in the range `[30, 100]`.

## Solutions (Rust)

### 1. Stack
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
