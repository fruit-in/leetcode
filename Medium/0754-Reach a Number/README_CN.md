# 754. 到达终点数字
在一根无限长的数轴上，你站在```0```的位置。终点在```target```的位置。

每次你可以选择向左或向右移动。第 n 次移动（从 1 开始），可以走 n 步。

返回到达终点需要的最小移动次数。

#### 示例 1:
<pre>
<strong>输入:</strong> target = 3
<strong>输出:</strong> 2
<strong>解释:</strong>
第一次移动，从 0 到 1 。
第二次移动，从 1 到 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = 2
<strong>输出:</strong> 3
<strong>解释:</strong>
第一次移动，从 0 到 1 。
第二次移动，从 1 到 -1 。
第三次移动，从 -1 到 2 。
</pre>

#### 注意:
* ```target```是在```[-10^9, 10^9]```范围中的非零整数。

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut target = target;
        let mut position = 0;
        let mut step = 0;
        if target < 0 {
            target = -target;
        }
        while position < target {
            step += 1;
            position += step;
        }
        if (position - target) % 2 == 0 {
            step
        } else if step % 2 == 0 {
            step + 1
        } else {
            step + 2
        }
    }
}
```
