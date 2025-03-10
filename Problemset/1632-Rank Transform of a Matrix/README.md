# 1632. Rank Transform of a Matrix
Given an `m x n` `matrix`, return *a new matrix* `answer` *where* `answer[row][col]` *is the **rank** of* `matrix[row][col]`.

The **rank** is an **integer** that represents how large an element is compared to other elements. It is calculated using the following rules:
* The rank is an integer starting from 1.
* If two elements `p` and `q` are in the **same row or column**, then:
    * If `p < q` then `rank(p) < rank(q)`
    * If `p == q` then `rank(p) == rank(q)`
    * If `p > q` then `rank(p) > rank(q)`
* The **rank** should be as **small** as possible.

The test cases are generated so that `answer` is unique under the given rules.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/10/18/rank1.jpg)
<pre>
<strong>Input:</strong> matrix = [[1,2],[3,4]]
<strong>Output:</strong> [[1,2],[2,3]]
<strong>Explanation:</strong>
The rank of matrix[0][0] is 1 because it is the smallest integer in its row and column.
The rank of matrix[0][1] is 2 because matrix[0][1] > matrix[0][0] and matrix[0][0] is rank 1.
The rank of matrix[1][0] is 2 because matrix[1][0] > matrix[0][0] and matrix[0][0] is rank 1.
The rank of matrix[1][1] is 3 because matrix[1][1] > matrix[0][1], matrix[1][1] > matrix[1][0], and both matrix[0][1] and matrix[1][0] are rank 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/10/18/rank2.jpg)
<pre>
<strong>Input:</strong> matrix = [[7,7],[7,7]]
<strong>Output:</strong> [[1,1],[1,1]]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/10/18/rank3.jpg)
<pre>
<strong>Input:</strong> matrix = [[20,-21,14],[-19,4,19],[22,-47,24],[-19,4,19]]
<strong>Output:</strong> [[4,2,3],[1,3,4],[5,1,6],[1,3,4]]
</pre>

#### Constraints:
* `m == matrix.length`
* `n == matrix[i].length`
* `1 <= m, n <= 500`
* <code>-10<sup>9</sup> <= matrix[row][col] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
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
