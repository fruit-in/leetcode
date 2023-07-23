# 1359. 有效的快递序列数目
给你 `n` 笔订单，每笔订单都需要快递服务。

请你统计所有有效的 收件/配送 序列的数目，确保第 `i` 个物品的配送服务 `delivery(i)` 总是在其收件服务 `pickup(i)` 之后。

由于答案可能很大，请返回答案对 `10^9 + 7` 取余的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 只有一种序列 (P1, D1)，物品 1 的配送服务（D1）在物品 1 的收件服务（P1）后。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 6
<strong>解释:</strong> 所有可能的序列包括：
(P1,P2,D1,D2)，(P1,P2,D2,D1)，(P1,D1,P2,D2)，(P2,P1,D1,D2)，(P2,P1,D2,D1) 和 (P2,D2,P1,D1)。
(P1,D2,P2,D1) 是一个无效的序列，因为物品 2 的收件服务（P2）不应在物品 2 的配送服务（D2）之后。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 90
</pre>

#### 提示:
* `1 <= n <= 500`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut ret = 1_i64;

        for i in 1..n as i64 {
            ret = (ret * (2 * i + 1) * (i + 1)) % 1_000_000_007;
        }

        ret as i32
    }
}
```
