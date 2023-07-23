class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        ret = []

        for ys in itertools.permutations(range(n)):
            board = [['.'] * n for _ in range(n)]
            for x, y in zip(range(n), ys):
                if any(board[x - i][y - i] == 'Q' for i in range(1, min(x, y) + 1)) or \
                        any(board[x - i][y + i] == 'Q' for i in range(1, min(x, n - y - 1) + 1)):
                    break
                board[x][y] = 'Q'
            else:
                ret.append([''.join(row) for row in board])

        return ret
