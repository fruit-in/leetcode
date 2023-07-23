# 121. 买卖股票的最佳时机
给定一个数组，它的第 *i* 个元素是一支给定股票第 *i* 天的价格。

如果你最多只允许完成一笔交易（即买入和卖出一支股票），设计一个算法来计算你所能获取的最大利润。

注意你不能在买入股票前卖出股票。

#### 示例 1:
<pre>
<strong>输入:</strong> [7,1,5,3,6,4]
<strong>输出:</strong> 5
<strong>解释:</strong> 在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
     注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [7,6,4,3,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 在这种情况下, 没有交易完成, 所以最大利润为 0。
</pre>

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for (i, buy) in prices.iter().enumerate() {
            for sell in &prices[(i + 1)..] {
                if sell - buy > profit {
                    profit = sell - buy;
                }
            }
        }
        profit
    }
}
```

### 2. 单次遍历
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min_price = std::i32::MAX;
        for n in prices {
            profit = profit.max(n - min_price);
            min_price = min_price.min(n);
        }
        profit
    }
}
```
