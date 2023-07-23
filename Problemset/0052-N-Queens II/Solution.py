class Solution:
    def totalNQueens(self, n: int) -> int:
        ret = 0

        for ys in itertools.permutations(range(n)):
            board = [[False] * n for _ in range(n)]
            for x, y in zip(range(n), ys):
                if any(board[x - i][y - i] for i in range(1, min(x, y) + 1)) or \
                        any(board[x - i][y + i] for i in range(1, min(x, n - y - 1) + 1)):
                    break
                board[x][y] = True
            else:
                ret += 1

        return ret
