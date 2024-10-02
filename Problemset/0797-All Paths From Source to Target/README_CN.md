# 797. 所有可能的路径
给你一个有 `n` 个节点的 **有向无环图（DAG）**，请你找出所有从节点 `0` 到节点 `n-1` 的路径并输出（**不要求按特定顺序**）

`graph[i]` 是一个从节点 `i` 可以访问的所有节点的列表（即从节点 `i` 到节点 `graph[i][j]`存在一条有向边）。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/28/all_1.jpg)
<pre>
<strong>输入:</strong> graph = [[1,2],[3],[3],[]]
<strong>输出:</strong> [[0,1,3],[0,2,3]]
<strong>解释:</strong> 有两条路径 0 -> 1 -> 3 和 0 -> 2 -> 3
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/09/28/all_2.jpg)
<pre>
<strong>输入:</strong> graph = [[4,3,1],[3,2,4],[3],[4],[]]
<strong>输出:</strong> [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
</pre>

#### 提示:
* `n == graph.length`
* `2 <= n <= 15`
* `0 <= graph[i][j] < n`
* `graph[i][j] != i`（即不存在自环）
* `graph[i]` 中的所有元素 **互不相同**
* 保证输入为 **有向无环图（DAG）**

## 题解 (Python)

### 1. 题解
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
