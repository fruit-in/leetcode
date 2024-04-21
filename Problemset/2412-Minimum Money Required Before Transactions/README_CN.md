# 2412. 完成所有交易的初始最少钱数
给你一个下标从 **0** 开始的二维整数数组 `transactions`，其中<code>transactions[i] = [cost<sub>i</sub>, cashback<sub>i</sub>]</code> 。

数组描述了若干笔交易。其中每笔交易必须以 **某种顺序** 恰好完成一次。在任意一个时刻，你有一定数目的钱 `money` ，为了完成交易 `i` ，<code>money >= cost<sub>i</sub></code> 这个条件必须为真。执行交易后，你的钱数 `money` 变成 <code>money - cost<sub>i</sub> + cashback<sub>i</sub></code> 。

请你返回 **任意一种** 交易顺序下，你都能完成所有交易的最少钱数 `money` 是多少。

#### 示例 1:
<pre>
<strong>输入:</strong> transactions = [[2,1],[5,0],[4,2]]
<strong>输出:</strong> 10
<strong>解释:</strong>
刚开始 money = 10 ，交易可以以任意顺序进行。
可以证明如果 money < 10 ，那么某些交易无法进行。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> transactions = [[3,0],[0,3]]
<strong>输出:</strong> 3
<strong>解释:</strong>
- 如果交易执行的顺序是 [[3,0],[0,3]] ，完成所有交易需要的最少钱数是 3 。
- 如果交易执行的顺序是 [[0,3],[3,0]] ，完成所有交易需要的最少钱数是 0 。
所以，刚开始钱数为 3 ，任意顺序下交易都可以全部完成。
</pre>

#### 提示:
* <code>1 <= transactions.length <= 10<sup>5</sup></code>
* `transactions[i].length == 2`
* <code>0 <= cost<sub>i</sub>, cashback<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut loss = 0;
        let mut max_cost = 0;

        for t in &transactions {
            loss += (t[0] - t[1]).max(0) as i64;
            max_cost = max_cost.max(t[0] - (t[0] - t[1]).max(0));
        }

        max_cost as i64 + loss
    }
}
```
