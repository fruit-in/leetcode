# 1782. Count Pairs Of Nodes
You are given an undirected graph defined by an integer `n`, the number of nodes, and a 2D integer array `edges`, the edges in the graph, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an **undirected** edge between <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>. You are also given an integer array `queries`.

Let `incident(a, b)` be defined as the **number of edges** that are connected to **either** node `a` or `b`.

The answer to the <code>j<sup>th</sup></code> query is the **number of pairs** of nodes `(a, b)` that satisfy **both** of the following conditions:
* `a < b`
* `incident(a, b) > queries[j]`

Return *an array* `answers` *such that* `answers.length == queries.length` *and* `answers[j]` *is the answer of the* <code>j<sup>th</sup></code> *query*.

Note that there can be **multiple edges** between the same two nodes.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/08/winword_2021-06-08_00-58-39.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[1,2],[2,4],[1,3],[2,3],[2,1]], queries = [2,3]
<strong>Output:</strong> [6,5]
<strong>Explanation:</strong> The calculations for incident(a, b) are shown in the table above.
The answers for each of the queries are as follows:
- answers[0] = 6. All the pairs have an incident(a, b) value greater than 2.
- answers[1] = 5. All the pairs except (3, 4) have an incident(a, b) value greater than 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 5, edges = [[1,5],[1,5],[3,4],[2,5],[1,3],[5,1],[2,3],[2,5]], queries = [1,2,3,4,5]
<strong>Output:</strong> [10,10,9,8,6]
</pre>

#### Constraints:
* <code>2 <= n <= 2 * 10<sup>4</sup></code>
* <code>1 <= edges.length <= 10<sup>5</sup></code>
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* `1 <= queries.length <= 20`
* `0 <= queries[j] < edges.length`

## Solutions (Python)

### 1. Solution
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
