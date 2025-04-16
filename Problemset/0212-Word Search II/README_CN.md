# 212. 单词搜索 II
给定一个 `m x n` 二维字符网格 `board` 和一个单词（字符串）列表 `words`， *返回所有二维网格上的单词* 。

单词必须按照字母顺序，通过 **相邻的单元格** 内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母在一个单词中不允许被重复使用。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/11/07/search1.jpg)
<pre>
<strong>输入:</strong> board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
<strong>输出:</strong> ["eat","oath"]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/11/07/search2.jpg)
<pre>
<strong>输入:</strong> board = [["a","b"],["c","d"]], words = ["abcb"]
<strong>输出:</strong> []
</pre>

#### 提示:
* `m == board.length`
* `n == board[i].length`
* `1 <= m, n <= 12`
* `board[i][j]` 是一个小写英文字母
* <code>1 <= words.length <= 3 * 10<sup>4</sup></code>
* `1 <= words[i].length <= 10`
* `words[i]` 由小写英文字母组成
* `words` 中的所有字符串互不相同

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        def dfs(i: int, j: int, root: dict) -> None:
            if (i, j) in visited or i < 0 or i >= m or j < 0 or j >= n or board[i][j] not in root:
                return None

            if root[board[i][j]][0] != "":
                ret.append(root[board[i][j]][0])
                root[board[i][j]][0] = ""

            visited.add((i, j))
            dfs(i - 1, j, root[board[i][j]][1])
            dfs(i + 1, j, root[board[i][j]][1])
            dfs(i, j - 1, root[board[i][j]][1])
            dfs(i, j + 1, root[board[i][j]][1])
            visited.remove((i, j))

        m, n = len(board), len(board[0])
        root = {}
        visited = set()
        ret = []

        for word in words:
            curr = root

            for i, c in enumerate(word):
                if c not in curr:
                    curr[c] = ["", {}]
                if i == len(word) - 1:
                    curr[c][0] = word
                curr = curr[c][1]

        for i in range(m):
            for j in range(n):
                dfs(i, j, root)

        return ret
```
