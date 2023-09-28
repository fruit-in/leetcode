# 2144. 打折购买糖果的最小开销
一家商店正在打折销售糖果。每购买 两个 糖果，商店会 **免费** 送一个糖果。

免费送的糖果唯一的限制是：它的价格需要小于等于购买的两个糖果价格的 **较小值** 。
* 比方说，总共有 `4` 个糖果，价格分别为 `1` ，`2` ，`3` 和 `4` ，一位顾客买了价格为 `2` 和 `3` 的糖果，那么他可以免费获得价格为 `1` 的糖果，但不能获得价格为 `4` 的糖果。

给你一个下标从 **0** 开始的整数数组 `cost` ，其中 `cost[i]` 表示第 `i` 个糖果的价格，请你返回获得 **所有** 糖果的 **最小** 总开销。

#### 示例 1:
<pre>
<strong>输入:</strong> cost = [1,2,3]
<strong>输出:</strong> 5
<strong>解释:</strong> 我们购买价格为 2 和 3 的糖果，然后免费获得价格为 1 的糖果。
总开销为 2 + 3 = 5 。这是开销最小的 唯一 方案。
注意，我们不能购买价格为 1 和 3 的糖果，并免费获得价格为 2 的糖果。
这是因为免费糖果的价格必须小于等于购买的 2 个糖果价格的较小值。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cost = [6,5,7,9,2,2]
<strong>输出:</strong> 23
<strong>解释:</strong> 最小总开销购买糖果方案为：
- 购买价格为 9 和 7 的糖果
- 免费获得价格为 6 的糖果
- 购买价格为 5 和 2 的糖果
- 免费获得价格为 2 的最后一个糖果
因此，最小总开销为 9 + 7 + 5 + 2 = 23 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> cost = [5,5]
<strong>输出:</strong> 10
<strong>解释:</strong> 由于只有 2 个糖果，我们需要将它们都购买，而且没有免费糖果。
所以总最小开销为 5 + 5 = 10 。
</pre>

#### 提示:
* `1 <= cost.length <= 100`
* `1 <= cost[i] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        let mut ret = 0;
        cost.sort_unstable_by(|a, b| b.cmp(a));

        for candies3 in cost.chunks(3) {
            ret += candies3[0];
            ret += candies3.get(1).unwrap_or(&0);
        }

        ret
    }
}
```