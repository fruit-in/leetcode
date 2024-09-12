# 1334. Find the City With the Smallest Number of Neighbors at a Threshold Distance
There are `n` cities numbered from `0` to `n-1`. Given the array `edges` where <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>, weight<sub>i</sub>]</code> represents a bidirectional and weighted edge between cities <code>from<sub>i</sub></code> and <code>to<sub>i</sub></code>, and given the integer `distanceThreshold`.

Return the city with the smallest number of cities that are reachable through some path and whose distance is **at most** `distanceThreshold`, If there are multiple such cities, return the city with the greatest number.

Notice that the distance of a path connecting cities ***i*** and ***j*** is equal to the sum of the edges' weights along that path.

#### Example 1:
![](https://assets.leetcode.com/uploads/2024/08/23/problem1334example1.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[0,1,3],[1,2,1],[1,3,4],[2,3,1]], distanceThreshold = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> The figure above describes the graph.
The neighboring cities at a distanceThreshold = 4 for each city are:
City 0 -> [City 1, City 2]
City 1 -> [City 0, City 2, City 3]
City 2 -> [City 0, City 1, City 3]
City 3 -> [City 1, City 2]
Cities 0 and 3 have 2 neighboring cities at a distanceThreshold = 4, but we have to return city 3 since it has the greatest number.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2024/08/23/problem1334example0.png)
<pre>
<strong>Input:</strong> n = 5, edges = [[0,1,2],[0,4,8],[1,2,3],[1,4,2],[2,3,1],[3,4,1]], distanceThreshold = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> The figure above describes the graph.
The neighboring cities at a distanceThreshold = 2 for each city are:
City 0 -> [City 1]
City 1 -> [City 0, City 4]
City 2 -> [City 3, City 4]
City 3 -> [City 2, City 4]
City 4 -> [City 1, City 2, City 3]
The city 0 has 1 neighboring city at a distanceThreshold = 2.
</pre>

#### Constraints:
* `2 <= n <= 100`
* `1 <= edges.length <= n * (n - 1) / 2`
* `edges[i].length == 3`
* <code>0 <= from<sub>i</sub> < to<sub>i</sub> < n</code>
* <code>1 <= weight<sub>i</sub>, distanceThreshold <= 10^4</code>
* All pairs <code>(from<sub>i</sub>, to<sub>i</sub>)</code> are distinct.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut to_cities = vec![vec![]; n];
        let mut min_reachable = usize::MAX;
        let mut ret = 0;

        for edge in &edges {
            if edge[2] <= distance_threshold {
                to_cities[edge[0] as usize].push((edge[1] as usize, edge[2]));
                to_cities[edge[1] as usize].push((edge[0] as usize, edge[2]));
            }
        }

        for i in 0..n {
            let mut heap = BinaryHeap::from([(0, i)]);
            let mut visited = HashSet::new();

            while let Some((weight, from)) = heap.pop() {
                visited.insert(from);

                for &(to, w) in &to_cities[from] {
                    if !visited.contains(&to) && -weight + w <= distance_threshold {
                        heap.push((weight - w, to));
                    }
                }
            }

            if visited.len() <= min_reachable {
                min_reachable = visited.len();
                ret = i;
            }
        }

        ret as i32
    }
}
```
