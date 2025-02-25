# 1928. Minimum Cost to Reach Destination in Time
There is a country of `n` cities numbered from `0` to `n - 1` where **all the cities are connected** by bi-directional roads. The roads are represented as a 2D integer array `edges` where <code>edges[i] = [x<sub>i</sub>, y<sub>i</sub>, time<sub>i</sub>]</code> denotes a road between cities <code>x<sub>i</sub></code> and <code>y<sub>i</sub></code> that takes <code>time<sub>i</sub></code> minutes to travel. There may be multiple roads of differing travel times connecting the same two cities, but no road connects a city to itself.

Each time you pass through a city, you must pay a passing fee. This is represented as a **0-indexed** integer array `passingFees` of length `n` where `passingFees[j]` is the amount of dollars you must pay when you pass through city `j`.

In the beginning, you are at city `0` and want to reach city `n - 1` in `maxTime` **minutes or less**. The **cost** of your journey is the **summation of passing fees** for each city that you passed through at some moment of your journey (**including** the source and destination cities).

Given `maxTime`, `edges`, and `passingFees`, return *the **minimum cost** to complete your journey, or* `-1` *if you cannot complete it within* `maxTime` *minutes*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/04/leetgraph1-1.png)
<pre>
<strong>Input:</strong> maxTime = 30, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
<strong>Output:</strong> 11
<strong>Explanation:</strong> The path to take is 0 -> 1 -> 2 -> 5, which takes 30 minutes and has $11 worth of passing fees.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/04/copy-of-leetgraph1-1.png)
<pre>
<strong>Input:</strong> maxTime = 29, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
<strong>Output:</strong> 48
<strong>Explanation:</strong> The path to take is 0 -> 3 -> 4 -> 5, which takes 26 minutes and has $48 worth of passing fees.
You cannot take path 0 -> 1 -> 2 -> 5 since it would take too long.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> maxTime = 25, edges = [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], passingFees = [5,1,2,20,20,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no way to reach city 5 from city 0 within 25 minutes.
</pre>

#### Constraints:
* `1 <= maxTime <= 1000`
* `n == passingFees.length`
* `2 <= n <= 1000`
* `n - 1 <= edges.length <= 1000`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1</code>
* <code>1 <= time<sub>i</sub> <= 1000</code>
* `1 <= passingFees[j] <= 1000`
* The graph may contain multiple edges between two nodes.
* The graph does not contain self loops.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let max_time = max_time as usize;
        let n = edges.iter().map(|edge| edge[0].max(edge[1])).max().unwrap() as usize + 1;
        let mut dp = vec![vec![i32::MAX; n]; max_time + 1];
        let mut ret = i32::MAX;
        dp[0][0] = passing_fees[0];

        for t in 0..=max_time {
            for edge in &edges {
                let (x, y, time) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);

                if t >= time {
                    if dp[t - time][y] != i32::MAX {
                        dp[t][x] = dp[t][x].min(dp[t - time][y] + passing_fees[x]);
                    }
                    if dp[t - time][x] != i32::MAX {
                        dp[t][y] = dp[t][y].min(dp[t - time][x] + passing_fees[y]);
                    }
                }
            }

            ret = ret.min(dp[t][n - 1]);
        }

        if ret == i32::MAX {
            return -1;
        }

        ret
    }
}
```
