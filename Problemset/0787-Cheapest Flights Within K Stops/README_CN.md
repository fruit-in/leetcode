# 787. K 站中转内最便宜的航班
有 `n` 个城市通过一些航班连接。给你一个数组 `flights` ，其中 <code>flights[i] = [from<sub>i</sub>, to<sub>i</sub>, price<sub>i</sub>]</code> ，表示该航班都从城市 <code>from<sub>i</sub></code> 开始，以价格 <code>price<sub>i</sub></code> 抵达 <code>to<sub>i</sub></code>。

现在给定所有的城市和航班，以及出发城市 `src` 和目的地 `dst`，你的任务是找到出一条最多经过 `k` 站中转的路线，使得从 `src` 到 `dst` 的 **价格最便宜** ，并返回该价格。 如果不存在这样的路线，则输出 `-1`。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-3drawio.png)
<pre>
<strong>输入:</strong> n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
<strong>输出:</strong> 700
<strong>解释:</strong> 城市航班图如上
从城市 0 到城市 3 经过最多 1 站的最佳路径用红色标记，费用为 100 + 600 = 700。
请注意，通过城市 [0, 1, 2, 3] 的路径更便宜，但无效，因为它经过了 2 站。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-1drawio.png)
<pre>
<strong>输入:</strong> n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
<strong>输出:</strong> 200
<strong>解释:</strong>
城市航班图如上
从城市 0 到城市 2 经过最多 1 站的最佳路径标记为红色，费用为 100 + 100 = 200。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-2drawio.png)
<pre>
<strong>输入:</strong> n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
<strong>输出:</strong> 500
<strong>解释:</strong>
城市航班图如上
从城市 0 到城市 2 不经过站点的最佳路径标记为红色，费用为 500。
</pre>

#### 提示:
* `1 <= n <= 100`
* `0 <= flights.length <= (n * (n - 1) / 2)`
* `flights[i].length == 3`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub> < n</code>
* <code>from<sub>i</sub> != to<sub>i</sub></code>
* <code>1 <= price<sub>i</sub> <= 10<sup>4</sup></code>
* 航班没有重复，且不存在自环
* `0 <= src, dst, k < n`
* `src != dst`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let k = k as usize;
        let mut to_cities = vec![vec![]; n];
        let mut deque = VecDeque::from([(src, 0)]);
        let mut min_prices = vec![vec![i32::MAX; n]; k + 2];
        let mut ret = -1;
        min_prices[0][src] = 0;

        for f in &flights {
            to_cities[f[0] as usize].push((f[1] as usize, f[2]));
        }

        while let Some((from, stops)) = deque.pop_front() {
            if stops > k {
                break;
            }

            for &(to, price) in &to_cities[from] {
                if min_prices[stops][from] + price < min_prices[stops + 1][to] {
                    min_prices[stops + 1][to] = min_prices[stops][from] + price;
                    deque.push_back((to, stops + 1));
                }
            }
        }

        for i in 0..=k + 1 {
            if min_prices[i][dst] != i32::MAX && (ret == -1 || min_prices[i][dst] < ret) {
                ret = min_prices[i][dst];
            }
        }

        ret
    }
}
```
