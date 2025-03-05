# 188. 买卖股票的最佳时机 IV
给你一个整数数组 `prices` 和一个整数 `k` ，其中 `prices[i]` 是某支给定的股票在第 `i` 天的价格。

设计一个算法来计算你所能获取的最大利润。你最多可以完成 `k` 笔交易。也就是说，你最多可以买 `k` 次，卖 `k` 次。

**注意：**你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。

#### 示例 1:
<pre>
<strong>输入:</strong> k = 2, prices = [2,4,1]
<strong>输出:</strong> 2
<strong>解释:</strong> 在第 1 天 (股票价格 = 2) 的时候买入，在第 2 天 (股票价格 = 4) 的时候卖出，这笔交易所能获得利润 = 4-2 = 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 2, prices = [3,2,6,5,0,3]
<strong>输出:</strong> 7
<strong>解释:</strong> 在第 2 天 (股票价格 = 2) 的时候买入，在第 3 天 (股票价格 = 6) 的时候卖出, 这笔交易所能获得利润 = 6-2 = 4 。
     随后，在第 5 天 (股票价格 = 0) 的时候买入，在第 6 天 (股票价格 = 3) 的时候卖出, 这笔交易所能获得利润 = 3-0 = 3 。
</pre>

#### 提示:
* `1 <= k <= 100`
* `1 <= prices.length <= 1000`
* `0 <= prices[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut dp = vec![[0, i32::MIN]; k + 1];
        let mut ret = 0;

        for i in 0..prices.len() {
            let mut tmp = dp.clone();

            for j in 0..=i.min(k) {
                if j < k {
                    tmp[j + 1][1] = tmp[j + 1][1].max(dp[j][0] - prices[i]);
                }
                tmp[j][0] = tmp[j][0].max(dp[j][1] + prices[i]);
                ret = ret.max(tmp[j][0]);
            }

            dp = tmp;
        }

        ret
    }
}
```
