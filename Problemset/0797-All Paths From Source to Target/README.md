# 797. All Paths From Source to Target
Given a directed acyclic graph (**DAG**) of `n` nodes labeled from `0` to `n - 1`, find all possible paths from node `0` to node `n - 1` and return them in **any order**.

The graph is given as follows: `graph[i]` is a list of all nodes you can visit from node `i` (i.e., there is a directed edge from node `i` to node `graph[i][j]`).

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/28/all_1.jpg)
<pre>
<strong>Input:</strong> graph = [[1,2],[3],[3],[]]
<strong>Output:</strong> [[0,1,3],[0,2,3]]
<strong>Explanation:</strong> There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/09/28/all_2.jpg)
<pre>
<strong>Input:</strong> graph = [[4,3,1],[3,2,4],[3],[4],[]]
<strong>Output:</strong> [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
</pre>

#### Constraints:
* `n == graph.length`
* `2 <= n <= 15`
* `0 <= graph[i][j] < n`
* `graph[i][j] != i` (i.e., there will be no self-loops).
* All the elements of `graph[i]` are **unique**.
* The input graph is **guaranteed** to be a **DAG**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def allPathsSourceTarget(self, graph: List[List[int]]) -> List[List[int]]:
        paths = [[0]]
        ret = []

        while paths != []:
            path = paths.pop()

            if path[-1] == len(graph) - 1:
                ret.append(path)
                continue

            for j in graph[path[-1]]:
                paths.append(path + [j])

        return ret
```
