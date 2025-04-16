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
