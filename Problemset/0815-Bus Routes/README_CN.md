# 815. 公交路线
给你一个数组 `routes` ，表示一系列公交线路，其中每个 `routes[i]` 表示一条公交线路，第 `i` 辆公交车将会在上面循环行驶。

* 例如，路线 `routes[0] = [1, 5, 7]` 表示第 `0` 辆公交车会一直按序列 `1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ...` 这样的车站路线行驶。

现在从 `source` 车站出发（初始时不在公交车上），要前往 `target` 车站。 期间仅可乘坐公交车。

求出 **最少乘坐的公交车数量** 。如果不可能到达终点车站，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> routes = [[1,2,7],[3,6,7]], source = 1, target = 6
<strong>输出:</strong> 2
<strong>解释:</strong> 最优策略是先乘坐第一辆公交车到达车站 7 , 然后换乘第二辆公交车到车站 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> routes = [[7,12],[4,5,15],[6],[15,19],[9,12,13]], source = 15, target = 12
<strong>输出:</strong> -1
</pre>

#### 提示:
* `1 <= routes.length <= 500`.
* <code>1 <= routes[i].length <= 10<sup>5</sup></code>
* `routes[i]` 中的所有值 **互不相同**
* <code>sum(routes[i].length) <= 10<sup>5</sup></code>
* <code>0 <= routes[i][j] < 10<sup>6</sup></code>
* <code>0 <= source, target < 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut routes = routes
            .into_iter()
            .map(|route| route.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let mut deque = VecDeque::new();
        let mut visited = HashSet::new();
        let mut neighbors = vec![vec![]; routes.len()];

        for i in 0..routes.len() {
            if routes[i].contains(&source) {
                deque.push_back((i, 1));
                visited.insert(i);
            }

            for j in i + 1..routes.len() {
                for stop in routes[i].iter() {
                    if routes[j].contains(stop) {
                        neighbors[i].push(j);
                        neighbors[j].push(i);
                        break;
                    }
                }
            }
        }

        while let Some((i, buses)) = deque.pop_front() {
            if routes[i].contains(&target) {
                return buses;
            }

            for &j in &neighbors[i] {
                if !visited.contains(&j) {
                    deque.push_back((j, buses + 1));
                    visited.insert(j);
                }
            }
        }

        -1
    }
}
```
