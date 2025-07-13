# 1976. Number of Ways to Arrive at Destination
You are in a city that consists of `n` intersections numbered from `0` to `n - 1` with **bi-directional** roads between some intersections. The inputs are generated such that you can reach any intersection from any other intersection and that there is at most one road between any two intersections.

You are given an integer `n` and a 2D integer array `roads` where <code>roads[i] = [u<sub>i</sub>, v<sub>i</sub>, time<sub>i</sub>]</code> means that there is a road between intersections <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> that takes <code>time<sub>i</sub></code> minutes to travel. You want to know in how many ways you can travel from intersection `0` to intersection `n - 1` in the **shortest amount of time**.

Return *the **number of ways** you can arrive at your destination in the **shortest amount of time***. Since the answer may be large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2025/02/14/1976_corrected.png)
<pre>
<strong>Input:</strong> n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The shortest amount of time it takes to go from intersection 0 to intersection 6 is 7 minutes.
The four ways to get there in 7 minutes are:
- 0 ➝ 6
- 0 ➝ 4 ➝ 6
- 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
- 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, roads = [[1,0,10]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one way to go from intersection 0 to intersection 1, and it takes 10 minutes.
</pre>

#### Constraints:
* `1 <= n <= 200`
* `n - 1 <= roads.length <= n * (n - 1) / 2`
* `roads[i].length == 3`
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> <= n - 1</code>
* <code>1 <= timei <= 10<sup>9</sup></code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* There is at most one road connecting any two intersections.
* You can reach any intersection from any other intersection.

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut min_time_count = vec![(i64::MAX, 0); n];
        min_time_count[0] = (0, 1);

        for road in &roads {
            neighbors[road[0] as usize].push((road[1] as usize, road[2] as i64));
            neighbors[road[1] as usize].push((road[0] as usize, road[2] as i64));
        }

        while let Some((Reverse(time), u)) = heap.pop() {
            if time > min_time_count[u].0 {
                continue;
            }

            for &(v, t) in &neighbors[u] {
                if time + t < min_time_count[v].0 {
                    min_time_count[v] = (time + t, min_time_count[u].1);
                    heap.push((Reverse(time + t), v));
                } else if time + t == min_time_count[v].0 {
                    min_time_count[v].1 =
                        (min_time_count[v].1 + min_time_count[u].1) % 1_000_000_007;
                }
            }
        }

        min_time_count[n - 1].1
    }
}
```
