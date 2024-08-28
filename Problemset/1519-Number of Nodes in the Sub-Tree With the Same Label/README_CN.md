# 1519. 子树中标签相同的节点数
给你一棵树（即，一个连通的无环无向图），这棵树由编号从 `0`  到 `n - 1` 的 n 个节点组成，且恰好有 `n - 1` 条 `edges` 。树的根节点为节点 `0` ，树上的每一个节点都有一个标签，也就是字符串 `labels` 中的一个小写字符（编号为 `i` 的 节点的标签就是 `labels[i]` ）

边数组 `edges` 以 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 的形式给出，该格式表示节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间存在一条边。

返回一个大小为 *`n`* 的数组，其中 `ans[i]` 表示第 `i` 个节点的子树中与节点 `i` 标签相同的节点数。

树 `T` 中的子树是由 `T` 中的某个节点及其所有后代节点组成的树。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/07/01/q3e1.jpg)
<pre>
<strong>输入:</strong> n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], labels = "abaedcd"
<strong>输出:</strong> [2,1,1,1,1,1,1]
<strong>解释:</strong> 节点 0 的标签为 'a' ，以 'a' 为根节点的子树中，节点 2 的标签也是 'a' ，因此答案为 2 。注意树中的每个节点都是这棵子树的一部分。
节点 1 的标签为 'b' ，节点 1 的子树包含节点 1、4 和 5，但是节点 4、5 的标签与节点 1 不同，故而答案为 1（即，该节点本身）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/07/01/q3e2.jpg)
<pre>
<strong>输入:</strong> n = 4, edges = [[0,1],[1,2],[0,3]], labels = "bbbb"
<strong>输出:</strong> [4,2,1,1]
<strong>解释:</strong> 节点 2 的子树中只有节点 2 ，所以答案为 1 。
节点 3 的子树中只有节点 3 ，所以答案为 1 。
节点 1 的子树中包含节点 1 和 2 ，标签都是 'b' ，因此答案为 2 。
节点 0 的子树中包含节点 0、1、2 和 3，标签都是 'b'，因此答案为 4 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/07/01/q3e3.jpg)
<pre>
<strong>输入:</strong> n = 5, edges = [[0,1],[0,2],[1,3],[0,4]], labels = "aabab"
<strong>输出:</strong> [3,2,1,1,1]
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `labels.length == n`
* `labels` 仅由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countSubTrees(self, n: int, edges: List[List[int]], labels: str) -> List[int]:
        children = [set() for _ in range(n)]
        parent = [-1] * n
        stack = [0]
        count = [[0] * 26 for _ in range(n)]
        ans = [0] * n

        for (a, b) in edges:
            children[a].add(b)
            children[b].add(a)

        while stack != []:
            i = stack.pop()

            for j in children[i]:
                children[j].remove(i)
                parent[j] = i
                stack.append(j)

        stack = [i for i in range(n) if len(children[i]) == 0]

        while stack != []:
            i = stack.pop()

            count[i][ord(labels[i]) - 97] += 1
            ans[i] = count[i][ord(labels[i]) - 97]

            if parent[i] != -1:
                for j in range(26):
                    count[parent[i]][j] += count[i][j]
                children[parent[i]].remove(i)
                if len(children[parent[i]]) == 0:
                    stack.append(parent[i])

        return ans
```
