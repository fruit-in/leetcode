# 1377. T 秒后青蛙的位置
给你一棵由 `n` 个顶点组成的无向树，顶点编号从 `1` 到 `n`。青蛙从 **顶点 1** 开始起跳。规则如下：

* 在一秒内，青蛙从它所在的当前顶点跳到另一个 **未访问** 过的顶点（如果它们直接相连）。
* 青蛙无法跳回已经访问过的顶点。
* 如果青蛙可以跳到多个不同顶点，那么它跳到其中任意一个顶点上的机率都相同。
* 如果青蛙不能跳到任何未访问过的顶点上，那么它每次跳跃都会停留在原地。

无向树的边用数组 `edges` 描述，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 意味着存在一条直接连通 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 两个顶点的边。

返回青蛙在 *`t`* 秒后位于目标顶点 *`target`* 上的概率。与实际答案相差不超过 <code>10<sup>-5</sup></code> 的结果将被视为正确答案。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/21/frog1.jpg)
<pre>
<strong>输入:</strong> n = 7, edges = [[1,2],[1,3],[1,7],[2,4],[2,6],[3,5]], t = 2, target = 4
<strong>输出:</strong> 0.16666666666666666
<strong>解释:</strong> 上图显示了青蛙的跳跃路径。青蛙从顶点 1 起跳，第 1 秒 有 1/3 的概率跳到顶点 2 ，然后第 2 秒 有 1/2 的概率跳到顶点 4，因此青蛙在 2 秒后位于顶点 4 的概率是 1/3 * 1/2 = 1/6 = 0.16666666666666666 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/21/frog2.jpg)
<pre>
<strong>输入:</strong> n = 7, edges = [[1,2],[1,3],[1,7],[2,4],[2,6],[3,5]], t = 1, target = 7
<strong>输出:</strong> 0.3333333333333333
<strong>解释:</strong> 上图显示了青蛙的跳跃路径。青蛙从顶点 1 起跳，有 1/3 = 0.3333333333333333 的概率能够 1 秒 后跳到顶点 7 。
</pre>

#### 提示:
* `1 <= n <= 100`
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub> <= n</code>
* `1 <= t <= 50`
* `1 <= target <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let n = n as usize;
        let mut parent = vec![0; n + 1];
        let mut children = vec![vec![]; n + 1];
        let mut nodes = vec![1];
        let mut curr = target as usize;
        let mut ret = 1.0;

        while let Some(node) = nodes.pop() {
            for edge in &edges {
                if edge[0] == node as i32 && edge[1] != parent[node] as i32 {
                    parent[edge[1] as usize] = node;
                    children[node].push(edge[1] as usize);
                    nodes.push(edge[1] as usize);
                } else if edge[1] == node as i32 && edge[0] != parent[node] as i32 {
                    parent[edge[0] as usize] = node;
                    children[node].push(edge[0] as usize);
                    nodes.push(edge[0] as usize);
                }
            }
        }

        for i in 0..=t {
            if parent[curr] == 0 {
                if i < t && !children[target as usize].is_empty() {
                    return 0.0;
                }

                return ret;
            }

            curr = parent[curr];
            ret /= children[curr].len() as f64;
        }

        0.0
    }
}
```
