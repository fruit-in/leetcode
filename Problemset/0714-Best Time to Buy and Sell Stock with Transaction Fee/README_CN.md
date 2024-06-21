# 714. 买卖股票的最佳时机含手续费
给定一个整数数组 `prices`，其中 `prices[i]`表示第 `i` 天的股票价格 ；整数 `fee` 代表了交易股票的手续费用。

你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。

返回获得利润的最大值。

**注意：**这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。

#### 示例 1:
<pre>
<strong>输入:</strong> prices = [1,3,2,8,4,9], fee = 2
<strong>输出:</strong> 8
<strong>解释:</strong> 能够达到的最大利润:
在此处买入 prices[0] = 1
在此处卖出 prices[3] = 8
在此处买入 prices[4] = 4
在此处卖出 prices[5] = 9
总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> prices = [1,3,7,5,10,3], fee = 3
<strong>输出:</strong> 6
</pre>

#### 提示:
* <code>1 <= prices.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= prices[i] < 5 * 10<sup>4</sup></code>
* <code>0 <= fee < 5 * 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut x, mut y) = (0, -prices[0] - fee);

        for i in 1..prices.len() {
            (x, y) = (x.max(y + prices[i]), y.max(x - prices[i] - fee));
        }

        x
    }
}
```
