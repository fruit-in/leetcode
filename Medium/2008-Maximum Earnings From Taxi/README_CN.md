# 2008. 出租车的最大盈利
你驾驶出租车行驶在一条有 `n` 个地点的路上。这 `n` 个地点从近到远编号为 `1` 到 `n` ，你想要从 `1` 开到 `n` ，通过接乘客订单盈利。你只能沿着编号递增的方向前进，不能改变方向。

乘客信息用一个下标从 **0** 开始的二维数组 `rides` 表示，其中 `rides[i] = [starti, endi, tipi]` 表示第 `i` 位乘客需要从地点 `starti` 前往 `endi` ，愿意支付 `tipi` 元的小费。

**每一位** 你选择接单的乘客 `i` ，你可以 **盈利** `endi - starti + tipi` 元。你同时 **最多** 只能接一个订单。

给你 `n` 和 `rides` ，请你返回在最优接单方案下，你能盈利 **最多** 多少元。

**注意:** 你可以在一个地点放下一位乘客，并在同一个地点接上另一位乘客。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5, rides = [[2,5,4],[1,5,1]]
<strong>输出:</strong> 7
<strong>解释:</strong> 我们可以接乘客 0 的订单，获得 5 - 2 + 4 = 7 元。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 20, rides = [[1,6,1],[3,10,2],[10,12,3],[11,12,2],[12,15,2],[13,18,1]]
<strong>输出:</strong> 20
<strong>解释:</strong> 我们可以接以下乘客的订单：
- 将乘客 1 从地点 3 送往地点 10 ，获得 10 - 3 + 2 = 9 元。
- 将乘客 2 从地点 10 送往地点 12 ，获得 12 - 10 + 3 = 5 元。
- 将乘客 5 从地点 13 送往地点 18 ，获得 18 - 13 + 1 = 6 元。
我们总共获得 9 + 5 + 6 = 20 元。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= rides.length <= 3 * 10<sup>4</sup></code>
* `rides[i].length == 3`
* <code>1 <= start<sub>i</sub> < end<sub>i</sub> <= n</code>
* <code>1 <= tip<sub>i</sub> <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut rides = rides;
        let mut i = 1;
        let mut dp = vec![0; n as usize + 1];
        rides.sort_unstable();

        for ride in rides {
            let (start, end, tip) = (ride[0], ride[1], ride[2]);

            for j in i..=start as usize {
                dp[j] = dp[j].max(dp[j - 1]);
            }

            i = start as usize + 1;
            dp[end as usize] =
                dp[end as usize].max(dp[start as usize] + (end - start + tip) as i64);
        }

        for j in i..=n as usize {
            dp[j] = dp[j].max(dp[j - 1]);
        }

        dp[n as usize]
    }
}
```
