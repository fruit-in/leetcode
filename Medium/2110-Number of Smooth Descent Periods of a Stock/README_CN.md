# 2110. 股票平滑下跌阶段的数目
给你一个整数数组 `prices` ，表示一支股票的历史每日股价，其中 `prices[i]` 是这支股票第 `i` 天的价格。

一个 **平滑下降的阶段** 定义为：对于 **连续一天或者多天** ，每日股价都比 **前一日股价恰好少** `1` ，这个阶段第一天的股价没有限制。

请你返回 **平滑下降阶段** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> prices = [3,2,1,4]
<strong>输出:</strong> 7
<strong>解释:</strong> 总共有 7 个平滑下降阶段：
[3], [2], [1], [4], [3,2], [2,1] 和 [3,2,1]
注意，仅一天按照定义也是平滑下降阶段。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> prices = [8,6,7,7]
<strong>输出:</strong> 4
<strong>解释:</strong> 总共有 4 个连续平滑下降阶段：[8], [6], [7] 和 [7]
由于 8 - 6 ≠ 1 ，所以 [8,6] 不是平滑下降阶段。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> prices = [1]
<strong>输出:</strong> 1
<strong>解释:</strong> 总共有 1 个平滑下降阶段：[1]
</pre>

#### 提示:
* <code>1 <= prices.length <= 10<sup>5</sup></code>
* <code>1 <= prices[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count = 1;
        let mut ret = 0;

        for i in 1..prices.len() {
            if prices[i] - prices[i - 1] == -1 {
                count += 1;
            } else {
                ret += count * (count + 1) / 2;
                count = 1;
            }
        }

        ret += count * (count + 1) / 2;

        ret
    }
}
```
