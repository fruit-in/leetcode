class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        emptycells = []
        rows = [0] * 9
        cols = [0] * 9
        boxes = [0] * 9
        i = 0

        for r in range(9):
            for c in range(9):
                if board[r][c] == '.':
                    board[r][c] = '0'
                    emptycells.append((r, c))
                else:
                    x = ord(board[r][c]) - 48
                    rows[r] |= 1 << x
                    cols[c] |= 1 << x
                    boxes[r // 3 * 3 + c // 3] |= 1 << x

        while i < len(emptycells):
            r, c = emptycells[i]
            mask = rows[r] | cols[c] | boxes[r // 3 * 3 + c // 3]
            x = ord(board[r][c]) - 47
            while x <= 9 and mask & (1 << x) != 0:
                x += 1
            if x <= 9:
                board[r][c] = chr(x + 48)
                i += 1
            else:
                board[r][c] = '0'
                i -= 1
                r, c = emptycells[i]
                x = ord(board[r][c]) - 48
            rows[r] ^= 1 << x
            cols[c] ^= 1 << x
            boxes[r // 3 * 3 + c // 3] ^= 1 << x
