# 122. 买卖股票的最佳时机 II
给定一个数组，它的第 *i* 个元素是一支给定股票第 *i* 天的价格。

设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。

**注意:** 你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

#### 示例 1:
<pre>
<strong>输入:</strong> [7,1,5,3,6,4]
<strong>输出:</strong> 7
<strong>解释:</strong> 在第 2 天（股票价格 = 1）的时候买入，在第 3 天（股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
     随后，在第 4 天（股票价格 = 3）的时候买入，在第 5 天（股票价格 = 6）的时候卖出, 这笔交易所能获得利润 = 6-3 = 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,3,4,5]
<strong>输出:</strong> 4
<strong>解释:</strong> 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4 。
     注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
     因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [7,6,4,3,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 在这种情况下, 没有交易完成, 所以最大利润为 0。
</pre>

## 题解 (Rust)

### 1. 对相邻两个数的差(> 0)求和
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            profit += 0.max(prices[i] - prices[i - 1]);
        }
        profit
    }
}
```

### 2. 峰谷法
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut i = 0;
        while i + 1 < prices.len() {
            while i + 1 < prices.len() && prices[i] >= prices[i + 1] {
                i += 1;
            }
            profit -= prices[i];
            while i + 1 < prices.len() && prices[i] <= prices[i + 1] {
                i += 1;
            }
            profit += prices[i];
        }
        profit
    }
}
```
