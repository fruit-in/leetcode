# 1632. 矩阵转换后的秩
给你一个 `m x n` 的矩阵 `matrix` ，请你返回一个新的矩阵 `answer` ，其中 `answer[row][col]` 是 `matrix[row][col]` 的秩。

每个元素的 **秩** 是一个整数，表示这个元素相对于其他元素的大小关系，它按照如下规则计算：
* 秩是从 1 开始的一个整数。
* 如果两个元素 `p` 和 `q` 在 **同一行** 或者 **同一列** ，那么：
    * 如果 `p < q` ，那么 `rank(p) < rank(q)`
    * 如果 `p == q` ，那么 `rank(p) == rank(q)`
    * 如果 `p > q` ，那么 `rank(p) > rank(q)`
* **秩** 需要越 **小** 越好。

题目保证按照上面规则 `answer` 数组是唯一的。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/10/18/rank1.jpg)
<pre>
<strong>输入:</strong> matrix = [[1,2],[3,4]]
<strong>输出:</strong> [[1,2],[2,3]]
<strong>解释:</strong>
matrix[0][0] 的秩为 1 ，因为它是所在行和列的最小整数。
matrix[0][1] 的秩为 2 ，因为 matrix[0][1] > matrix[0][0] 且 matrix[0][0] 的秩为 1 。
matrix[1][0] 的秩为 2 ，因为 matrix[1][0] > matrix[0][0] 且 matrix[0][0] 的秩为 1 。
matrix[1][1] 的秩为 3 ，因为 matrix[1][1] > matrix[0][1]， matrix[1][1] > matrix[1][0] 且 matrix[0][1] 和 matrix[1][0] 的秩都为 2 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/10/18/rank2.jpg)
<pre>
<strong>输入:</strong> matrix = [[7,7],[7,7]]
<strong>输出:</strong> [[1,1],[1,1]]
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/10/18/rank3.jpg)
<pre>
<strong>输入:</strong> matrix = [[20,-21,14],[-19,4,19],[22,-47,24],[-19,4,19]]
<strong>输出:</strong> [[4,2,3],[1,3,4],[5,1,6],[1,3,4]]
</pre>

#### 提示:
* `m == matrix.length`
* `n == matrix[i].length`
* `1 <= m, n <= 500`
* <code>-10<sup>9</sup> <= matrix[row][col] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def matrixRankTransform(self, matrix: List[List[int]]) -> List[List[int]]:
        m, n = len(matrix), len(matrix[0])
        ranksr = [0] * m
        ranksc = [0] * n
        cells = sorted((matrix[r][c], r, c)
                       for r in range(m) for c in range(n))
        parent = {}
        answermap = {}

        for i in range(len(cells)):
            r, c = cells[i][1], cells[i][2]
            parent[(r, c)] = (r, c)
            answermap[(r, c)] = max(ranksr[r], ranksc[c]) + 1

            if i + 1 < len(cells) and cells[i][0] == cells[i + 1][0]:
                continue

            rowparent = {}
            colparent = {}

            for (r, c) in parent:
                if r not in rowparent:
                    rowparent[r] = (r, c)
                while rowparent[r] != parent[rowparent[r]]:
                    rowparent[r] = parent[rowparent[r]]
                if answermap[(r, c)] <= answermap[rowparent[r]]:
                    parent[(r, c)] = rowparent[r]
                else:
                    parent[rowparent[r]] = (r, c)
                if c not in colparent:
                    colparent[c] = (r, c)
                while colparent[c] != parent[colparent[c]]:
                    colparent[c] = parent[colparent[c]]
                if answermap[parent[(r, c)]] <= answermap[colparent[c]]:
                    parent[parent[(r, c)]] = colparent[c]
                else:
                    parent[colparent[c]] = parent[(r, c)]

            for (r, c) in parent:
                while parent[(r, c)] != parent[parent[(r, c)]]:
                    parent[(r, c)] = parent[parent[(r, c)]]
                answermap[(r, c)] = answermap[parent[(r, c)]]
                ranksr[r] = answermap[(r, c)]
                ranksc[c] = answermap[(r, c)]

            parent = {}

        return [[answermap[(r, c)] for c in range(n)] for r in range(m)]
```
