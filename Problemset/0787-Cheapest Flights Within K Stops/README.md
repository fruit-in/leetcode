# 787. Cheapest Flights Within K Stops
There are `n` cities connected by some number of flights. You are given an array `flights` where <code>flights[i] = [from<sub>i</sub>, to<sub>i</sub>, price<sub>i</sub>]</code> indicates that there is a flight from city <code>from<sub>i</sub></code> to city <code>to<sub>i</sub></code> with cost <code>price<sub>i</sub></code>.

You are also given three integers `src`, `dst`, and `k`, return ***the cheapest price** from* `src` *to* `dst` *with at most* `k` *stops*. If there is no such route, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-3drawio.png)
<pre>
<strong>Input:</strong> n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
<strong>Output:</strong> 700
<strong>Explanation:</strong>
The graph is shown above.
The optimal path with at most 1 stop from city 0 to 3 is marked in red and has cost 100 + 600 = 700.
Note that the path through cities [0,1,2,3] is cheaper but is invalid because it uses 2 stops.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-1drawio.png)
<pre>
<strong>Input:</strong> n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
<strong>Output:</strong> 200
<strong>Explanation:</strong>
The graph is shown above.
The optimal path with at most 1 stop from city 0 to 2 is marked in red and has cost 100 + 100 = 200.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-2drawio.png)
<pre>
<strong>Input:</strong> n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
<strong>Output:</strong> 500
<strong>Explanation:</strong>
The graph is shown above.
The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.
</pre>

#### Constraints:
* `1 <= n <= 100`
* `0 <= flights.length <= (n * (n - 1) / 2)`
* `flights[i].length == 3`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub> < n</code>
* <code>from<sub>i</sub> != to<sub>i</sub></code>
* <code>1 <= price<sub>i</sub> <= 10<sup>4</sup></code>
* There will not be any multiple flights between two cities.
* `0 <= src, dst, k < n`
* `src != dst`

## Solutions (Rust)

### 1. Solution
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
