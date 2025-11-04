class Solution:
    def minDays(self, grid: List[List[int]]) -> int:
        def isDisconnected(grid: List[List[int]]) -> bool:
            visited = set()

            for i in range(len(grid)):
                for j in range(len(grid[0])):
                    if grid[i][j] == 0 or (i, j) in visited:
                        continue

                    if visited:
                        return True

                    stack = [(i, j)]
                    visited.add((i, j))

                    while stack:
                        r, c = stack.pop()
                        if r > 0 and grid[r - 1][c] == 1 and (r - 1, c) not in visited:
                            stack.append((r - 1, c))
                            visited.add((r - 1, c))
                        if r < len(grid) - 1 and grid[r + 1][c] == 1 and (r + 1, c) not in visited:
                            stack.append((r + 1, c))
                            visited.add((r + 1, c))
                        if c > 0 and grid[r][c - 1] == 1 and (r, c - 1) not in visited:
                            stack.append((r, c - 1))
                            visited.add((r, c - 1))
                        if c < len(grid[0]) - 1 and grid[r][c + 1] == 1 and (r, c + 1) not in visited:
                            stack.append((r, c + 1))
                            visited.add((r, c + 1))

            return not visited

        if isDisconnected(grid):
            return 0

        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if grid[i][j] == 0:
                    continue

                grid[i][j] = 0
                if isDisconnected(grid):
                    return 1
                grid[i][j] = 1

        return 2
