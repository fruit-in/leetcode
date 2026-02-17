# 1466. Reorder Routes to Make All Paths Lead to the City Zero
There are `n` cities numbered from `0` to `n - 1` and `n - 1` roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.

Roads are represented by `connections` where <code>connections[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> represents a road from city <code>a<sub>i</sub></code> to city <code>b<sub>i</sub></code>.

This year, there will be a big event in the capital (city `0`), and many people want to travel to this city.

Your task consists of reorienting some roads such that each city can visit the city `0`. Return the **minimum** number of edges changed.

It's **guaranteed** that each city can reach city `0` after reorder.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/05/13/sample_1_1819.png)
<pre>
<strong>Input:</strong> n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Change the direction of edges show in red such that each node can reach the node 0 (capital).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/05/13/sample_2_1819.png)
<pre>
<strong>Input:</strong> n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Change the direction of edges show in red such that each node can reach the node 0 (capital).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3, connections = [[1,0],[2,0]]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>2 <= n <= 5 * 10<sup>4</sup></code>
* `connections.length == n - 1`
* `connections[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut from_to = vec![vec![]; n];
        let mut to_from = vec![vec![]; n];
        let mut stack = vec![0];
        let mut visited = vec![false; n];
        visited[0] = true;
        let mut ret = 0;

        for connection in &connections {
            let (a, b) = (connection[0] as usize, connection[1] as usize);
            from_to[a].push(b);
            to_from[b].push(a);
        }

        while let Some(a) = stack.pop() {
            for &b in &from_to[a] {
                if !visited[b] {
                    stack.push(b);
                    visited[b] = true;
                    ret += 1;
                }
            }
            for &b in &to_from[a] {
                if !visited[b] {
                    stack.push(b);
                    visited[b] = true;
                }
            }
        }

        ret
    }
}
```
