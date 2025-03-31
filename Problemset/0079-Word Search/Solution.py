class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m, n = len(board), len(board[0])

        for r in range(m):
            for c in range(n):
                if board[r][c] != word[0]:
                    continue

                visited = {(r, c)}
                stack = [[r, c, 0, 0]]

                while stack != []:
                    i, j, k, l = stack[-1]

                    if k == len(word) - 1:
                        return True

                    if l == 0:
                        stack[-1][3] += 1
                        if i > 0 and (i - 1, j) not in visited and board[i - 1][j] == word[k + 1]:
                            visited.add((i - 1, j))
                            stack.append([i - 1, j, k + 1, 0])
                    elif l == 1:
                        stack[-1][3] += 1
                        if i < m - 1 and (i + 1, j) not in visited and board[i + 1][j] == word[k + 1]:
                            visited.add((i + 1, j))
                            stack.append([i + 1, j, k + 1, 0])
                    elif l == 2:
                        stack[-1][3] += 1
                        if j > 0 and (i, j - 1) not in visited and board[i][j - 1] == word[k + 1]:
                            visited.add((i, j - 1))
                            stack.append([i, j - 1, k + 1, 0])
                    elif l == 3:
                        stack[-1][3] += 1
                        if j < n - 1 and (i, j + 1) not in visited and board[i][j + 1] == word[k + 1]:
                            visited.add((i, j + 1))
                            stack.append([i, j + 1, k + 1, 0])
                    else:
                        stack.pop()
                        visited.remove((i, j))

        return False
