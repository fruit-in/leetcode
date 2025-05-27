# 834. 树中距离之和
给定一个无向、连通的树。树中有 `n` 个标记为 `0...n-1` 的节点以及 `n-1` 条边 。

给定整数 `n` 和数组 `edges` ， <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code>表示树中的节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条边。

返回长度为 `n` 的数组 `answer` ，其中 `answer[i]` 是树中第 `i` 个节点与所有其他节点之间的距离之和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist1.jpg)
<pre>
<strong>输入:</strong> n = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
<strong>输出:</strong> [8,12,6,10,10,10]
<strong>解释:</strong> 树如图所示。
我们可以计算出 dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
也就是 1 + 1 + 2 + 2 + 2 = 8。 因此，answer[0] = 8，以此类推。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist2.jpg)
<pre>
<strong>输入:</strong> n = 1, edges = []
<strong>输出:</strong> [0]
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist3.jpg)
<pre>
<strong>输入:</strong> n = 2, edges = [[1,0]]
<strong>输出:</strong> [1,1]
</pre>

#### 提示:
* <code>1 <= n <= 3 * 10<sup>4</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 给定的输入保证为有效的树

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut children = vec![vec![]; n];
        let mut parent = vec![n; n];
        let mut stack = vec![0];
        let mut indegree = vec![0; n];
        let mut subtree_nodes_count = vec![1; n];
        let mut answer = vec![0; n];

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            children[a].push(b);
            children[b].push(a);
            indegree[a] += 1;
            indegree[b] += 1;
        }

        while let Some(i) = stack.pop() {
            for &j in &children[i] {
                if j != 0 && parent[j] == n {
                    parent[j] = i;
                    stack.push(j);
                }
            }
        }

        for i in 1..n {
            indegree[i] -= 1;
            if children[i].len() == 1 {
                children[i].pop();
                stack.push(i);
                continue;
            }

            for j in 0..children[i].len() {
                if children[i][j] == parent[i] {
                    children[i].swap_remove(j);
                    break;
                }
            }
        }

        while let Some(i) = stack.pop() {
            if i == 0 {
                break;
            }

            indegree[parent[i]] -= 1;
            if indegree[parent[i]] == 0 {
                stack.push(parent[i]);
            }
            subtree_nodes_count[parent[i]] += subtree_nodes_count[i];
            answer[parent[i]] += answer[i] + subtree_nodes_count[i];
        }

        stack.push(0);
        while let Some(i) = stack.pop() {
            for &j in &children[i] {
                answer[j] = answer[i] - subtree_nodes_count[j] * 2 + n as i32;
                stack.push(j);
            }
        }

        answer
    }
}
```
