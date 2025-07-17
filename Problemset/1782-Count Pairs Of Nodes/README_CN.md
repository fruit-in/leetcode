# 1782. 统计点对的数目
给你一个无向图，无向图由整数 `n`  ，表示图中节点的数目，和 `edges` 组成，其中 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间有一条无向边。同时给你一个代表查询的整数数组 `queries` 。

第 `j` 个查询的答案是满足如下条件的点对 `(a, b)` 的数目：
* `a < b`
* `cnt` 是与 `a` **或者** `b` 相连的边的数目，且 `cnt` **严格大于** `queries[j]` 。

请你返回一个数组 `answers` ，其中 `answers.length == queries.length` 且 `answers[j]` 是第 `j` 个查询的答案。

请注意，图中可能会有 **多重边** 。

#### 示例 1:
![](https://pic.leetcode.cn/1692844033-Kvxjvr-image.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[1,2],[2,4],[1,3],[2,3],[2,1]], queries = [2,3]
<strong>输出:</strong> [6,5]
<strong>解释:</strong> 每个点对中，与至少一个点相连的边的数目如上图所示。
answers[0] = 6。所有的点对(a, b)中边数和都大于2，故有6个；
answers[1] = 5。所有的点对(a, b)中除了(3,4)边数等于3，其它点对边数和都大于3，故有5个。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 5, edges = [[1,5],[1,5],[3,4],[2,5],[1,3],[5,1],[2,3],[2,5]], queries = [1,2,3,4,5]
<strong>输出:</strong> [10,10,9,8,6]
</pre>

#### 提示:
* <code>2 <= n <= 2 * 10<sup>4</sup></code>
* <code>1 <= edges.length <= 10<sup>5</sup></code>
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* `1 <= queries.length <= 20`
* `0 <= queries[j] < edges.length`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countPairs(self, n: int, edges: List[List[int]], queries: List[int]) -> List[int]:
        degree = [0] * n
        multiplecount = {}
        answers = [0] * len(queries)

        for u, v in edges:
            u, v = min(u, v) - 1, max(u, v) - 1
            degree[u] += 1
            degree[v] += 1
            if (u, v) not in multiplecount:
                multiplecount[(u, v)] = 0
            multiplecount[(u, v)] += 1

        sorteddegree = sorted(degree)

        for i in range(len(queries)):
            for j in range(n):
                answers[i] += n - \
                    bisect_right(sorteddegree, queries[i] - degree[j])
                if degree[j] * 2 > queries[i]:
                    answers[i] -= 1
            answers[i] //= 2

            for (u, v), c in multiplecount.items():
                if degree[u] + degree[v] > queries[i] and degree[u] + degree[v] - c <= queries[i]:
                    answers[i] -= 1

        return answers
```
