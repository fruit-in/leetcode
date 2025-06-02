# 1697. 检查边长度限制的路径是否存在
给你一个 `n` 个点组成的无向图边集 `edgeList` ，其中 <code>edgeList[i] = [u<sub>i</sub>, v<sub>i</sub>, dis<sub>i</sub>]</code> 表示点 <code>u<sub>i</sub></code> 和点 <code>v<sub>i</sub></code> 之间有一条长度为 <code>dis<sub>i</sub></code> 的边。请注意，两个点之间可能有 **超过一条边** 。

给你一个查询数组`queries` ，其中 <code>queries[j] = [p<sub>j</sub>, q<sub>j</sub>, limit<sub>j</sub>]</code> ，你的任务是对于每个查询 `queries[j]` ，判断是否存在从 <code>p<sub>j</sub></code> 到 <code>q<sub>j</sub></code> 的路径，且这条路径上的每一条边都 **严格小于** <code>limit<sub>j</sub></code> 。

请你返回一个 **布尔数组** `answer` ，其中 `answer.length == queries.length` ，当 `queries[j]` 的查询结果为 `true` 时， `answer` 第 `j` 个值为 `true` ，否则为 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/12/08/h.png)
<pre>
<strong>输入:</strong> n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]
<strong>输出:</strong> [false,true]
<strong>解释:</strong> 上图为给定的输入数据。注意到 0 和 1 之间有两条重边，分别为 2 和 16 。
对于第一个查询，0 和 1 之间没有小于 2 的边，所以我们返回 false 。
对于第二个查询，有一条路径（0 -> 1 -> 2）两条边都小于 5 ，所以这个查询我们返回 true 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/12/08/q.png)
<pre>
<strong>输入:</strong> n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]
<strong>输出:</strong> [true,false]
<strong>解释:</strong> 上图为给定数据。
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= edgeList.length, queries.length <= 10<sup>5</sup></code>
* `edgeList[i].length == 3`
* `queries[j].length == 3`
* <code>0 <= u<sub>i</sub>, v<sub>i</sub>, p<sub>j</sub>, q<sub>j</sub> <= n - 1</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* <code>p<sub>j</sub> != q<sub>j</sub></code>
* <code>1 <= dis<sub>i</sub>, limit<sub>j</sub> <= 10<sup>9</sup></code>
* 两个点之间可能有 **多条** 边。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def distanceLimitedPathsExist(self, n: int, edgeList: List[List[int]], queries: List[List[int]]) -> List[bool]:
        parent = list(range(n))
        indices = sorted(range(len(queries)), key=lambda i: queries[i][2])
        i = 0
        answer = [False] * len(queries)
        edgeList.sort(key=lambda e: e[2])

        for j in indices:
            while i < len(edgeList) and edgeList[i][2] < queries[j][2]:
                u, v, _ = edgeList[i]
                while parent[u] != parent[parent[u]]:
                    parent[u] = parent[parent[u]]
                while parent[v] != parent[parent[v]]:
                    parent[v] = parent[parent[v]]
                parent[parent[u]] = parent[v]
                i += 1

            p, q, _ = queries[j]
            while parent[p] != parent[parent[p]]:
                parent[p] = parent[parent[p]]
            while parent[q] != parent[parent[q]]:
                parent[q] = parent[parent[q]]

            answer[j] = parent[p] == parent[q]

        return answer
```
