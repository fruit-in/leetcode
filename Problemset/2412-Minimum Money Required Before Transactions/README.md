# 2412. Minimum Money Required Before Transactions
You are given a **0-indexed** 2D integer array `transactions`, where <code>transactions[i] = [cost<sub>i</sub>, cashback<sub>i</sub>]</code>.

The array describes transactions, where each transaction must be completed exactly once in **some order**. At any given moment, you have a certain amount of `money`. In order to complete transaction `i`, <code>money >= cost<sub>i</sub></code> must hold true. After performing a transaction, `money` becomes <code>money - cost<sub>i</sub> + cashback<sub>i</sub></code>.

Return *the minimum amount of* `money` *required before any transaction so that all of the transactions can be completed **regardless of the order** of the transactions*.

#### Example 1:
<pre>
<strong>Input:</strong> transactions = [[2,1],[5,0],[4,2]]
<strong>Output:</strong> 10
<strong>Explanation:</strong>
Starting with money = 10, the transactions can be performed in any order.
It can be shown that starting with money < 10 will fail to complete all transactions in some order.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> transactions = [[3,0],[0,3]]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- If transactions are in the order [[3,0],[0,3]], the minimum money required to complete the transactions is 3.
- If transactions are in the order [[0,3],[3,0]], the minimum money required to complete the transactions is 0.
Thus, starting with money = 3, the transactions can be performed in any order.
</pre>

#### Constraints:
* <code>1 <= transactions.length <= 10<sup>5</sup></code>
* `transactions[i].length == 2`
* <code>0 <= cost<sub>i</sub>, cashback<sub>i</sub> <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
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
