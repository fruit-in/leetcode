# 1466. 重新规划路线
`n` 座城市，从 `0` 到 `n-1` 编号，其间共有 `n-1` 条路线。因此，要想在两座不同城市之间旅行只有唯一一条路线可供选择（路线网形成一颗树）。去年，交通运输部决定重新规划路线，以改变交通拥堵的状况。

路线用 `connections` 表示，其中 `connections[i] = [a, b]` 表示从城市 `a` 到 `b` 的一条有向路线。

今年，城市 0 将会举办一场大型比赛，很多游客都想前往城市 0 。

请你帮助重新规划路线方向，使每个城市都可以访问城市 0 。返回需要变更方向的最小路线数。

题目数据 **保证** 每个城市在重新规划路线方向后都能到达城市 0 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/05/13/sample_1_1819.png)
<pre>
<strong>输入:</strong> n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
<strong>输出:</strong> 3
<strong>解释:</strong> 更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/05/13/sample_2_1819.png)
<pre>
<strong>输入:</strong> n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
<strong>输出:</strong> 2
<strong>解释:</strong> 更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3, connections = [[1,0],[2,0]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>2 <= n <= 5 * 10<sup>4</sup></code>
* `connections.length == n - 1`
* `connections[i].length == 2`
* `0 <= connections[i][0], connections[i][1] <= n - 1`
* `connections[i][0] != connections[i][1]`

## 题解 (Rust)

### 1. 题解
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
