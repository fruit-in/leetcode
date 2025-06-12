# 2242. 节点序列的最大得分
给你一个 `n` 个节点的 **无向图** ，节点编号为 `0` 到 `n - 1` 。

给你一个下标从 **0** 开始的整数数组 `scores` ，其中 `scores[i]` 是第 `i` 个节点的分数。同时给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> ，表示节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条 **无向** 边。

一个合法的节点序列如果满足以下条件，我们称它是 **合法的** ：
* 序列中每 **相邻** 节点之间有边相连。
* 序列中没有节点出现超过一次。

节点序列的分数定义为序列中节点分数之 **和** 。

请你返回一个长度为 `4` 的合法节点序列的最大分数。如果不存在这样的序列，请你返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/04/15/ex1new3.png)
<pre>
<strong>输入:</strong> scores = [5,2,9,8,4], edges = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
<strong>输出:</strong> 24
<strong>解释:</strong> 上图为输入的图，节点序列为 [0,1,2,3] 。
节点序列的分数为 5 + 2 + 9 + 8 = 24 。
观察可知，没有其他节点序列得分和超过 24 。
注意节点序列 [3,1,2,0] 和 [1,0,2,3] 也是合法的，且分数为 24 。
序列 [0,3,2,4] 不是合法的，因为没有边连接节点 0 和 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/17/ex2.png)
<pre>
<strong>输入:</strong> scores = [9,20,6,4,11,12], edges = [[0,3],[5,3],[2,4],[1,3]]
<strong>输出:</strong> -1
<strong>解释:</strong> 上图为输入的图。
没有长度为 4 的合法序列，所以我们返回 -1 。
</pre>

#### 提示:
* `n == scores.length`
* <code>4 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= scores[i] <= 10<sup>8</sup></code>
* <code>0 <= edges.length <= 5 * 10<sup>4</sup></code>
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 不会有重边。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maximumScore(self, scores: List[int], edges: List[List[int]]) -> int:
        n = len(scores)
        neighbors = [[] for _ in range(n)]
        top3 = [[] for _ in range(n)]
        ret = -1

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for a in range(n):
            top3[a] = sorted(neighbors[a], key=lambda b: scores[b])[-3:]

        for a, b in edges:
            top3a = [i for i in top3[a] if i != b]
            top3b = [i for i in top3[b] if i != a]

            if top3a == [] or top3b == []:
                continue
            elif top3a[-1] != top3b[-1]:
                ret = max(ret, scores[a] + scores[b] +
                          scores[top3a[-1]] + scores[top3b[-1]])
            else:
                if len(top3a) > 1:
                    ret = max(ret, scores[a] + scores[b] +
                              scores[top3a[-2]] + scores[top3b[-1]])
                if len(top3b) > 1:
                    ret = max(ret, scores[a] + scores[b] +
                              scores[top3a[-1]] + scores[top3b[-2]])

        return ret
```
