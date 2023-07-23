# 2240. 买钢笔和铅笔的方案数
给你一个整数 `total` ，表示你拥有的总钱数。同时给你两个整数 `cost1` 和 `cost2` ，分别表示一支钢笔和一支铅笔的价格。你可以花费你部分或者全部的钱，去买任意数目的两种笔。

请你返回购买钢笔和铅笔的 **不同方案数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> total = 20, cost1 = 10, cost2 = 5
<strong>输出:</strong> 9
<strong>解释:</strong> 一支钢笔的价格为 10 ，一支铅笔的价格为 5 。
- 如果你买 0 支钢笔，那么你可以买 0 ，1 ，2 ，3 或者 4 支铅笔。
- 如果你买 1 支钢笔，那么你可以买 0 ，1 或者 2 支铅笔。
- 如果你买 2 支钢笔，那么你没法买任何铅笔。
所以买钢笔和铅笔的总方案数为 5 + 3 + 1 = 9 种。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> total = 5, cost1 = 10, cost2 = 10
<strong>输出:</strong> 1
<strong>解释:</strong> 钢笔和铅笔的价格都为 10 ，都比拥有的钱数多，所以你没法购买任何文具。所以只有 1 种方案：买 0 支钢笔和 0 支铅笔。
</pre>

#### 提示:
* <code>1 <= total, cost1, cost2 <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let expensive = cost1.max(cost2);
        let cheap = cost1.min(cost2);

        (0..=total)
            .step_by(expensive as usize)
            .map(|x| ((total - x) / cheap) as i64 + 1)
            .sum()
    }
}
```
