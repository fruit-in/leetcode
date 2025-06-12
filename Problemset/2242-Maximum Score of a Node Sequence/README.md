# 2242. Maximum Score of a Node Sequence
There is an **undirected** graph with `n` nodes, numbered from `0` to `n - 1`.

You are given a **0-indexed** integer array `scores` of length `n` where `scores[i]` denotes the score of node `i`. You are also given a 2D integer array `edges` where edges[i] = [ai, bi] denotes that there exists an **undirected** edge connecting nodes ai and bi.

A node sequence is **valid** if it meets the following conditions:
* There is an edge connecting every pair of **adjacent** nodes in the sequence.
* No node appears more than once in the sequence.

The score of a node sequence is defined as the **sum** of the scores of the nodes in the sequence.

Return *the **maximum score** of a valid node sequence with a length of* `4`. If no such sequence exists, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/04/15/ex1new3.png)
<pre>
<strong>Input:</strong> scores = [5,2,9,8,4], edges = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
<strong>Output:</strong> 24
<strong>Explanation:</strong> The figure above shows the graph and the chosen node sequence [0,1,2,3].
The score of the node sequence is 5 + 2 + 9 + 8 = 24.
It can be shown that no other node sequence has a score of more than 24.
Note that the sequences [3,1,2,0] and [1,0,2,3] are also valid and have a score of 24.
The sequence [0,3,2,4] is not valid since no edge connects nodes 0 and 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/17/ex2.png)
<pre>
<strong>Input:</strong> scores = [9,20,6,4,11,12], edges = [[0,3],[5,3],[2,4],[1,3]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The figure above shows the graph.
There are no valid node sequences of length 4, so we return -1.
</pre>

#### Constraints:
* `n == scores.length`
* <code>4 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= scores[i] <= 10<sup>8</sup></code>
* <code>0 <= edges.length <= 5 * 10<sup>4</sup></code>
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* There are no duplicate edges.

## Solutions (Python)

### 1. Solution
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
