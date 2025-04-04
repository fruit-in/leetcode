class Solution:
    def uniquePathsIII(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        nonobstaclecount = 0
        stack = []
        visited = set()
        ret = 0

        for i in range(m):
            for j in range(n):
                if grid[i][j] == 1:
                    stack.append([i, j, 0])
                    visited.add((i, j))
                if grid[i][j] != -1:
                    nonobstaclecount += 1

        while stack != []:
            i, j, direct = stack[-1]

            if grid[i][j] == 2:
                ret += len(visited) == nonobstaclecount
                stack.pop()
                visited.remove((i, j))
            elif direct == 0:
                stack[-1][2] += 1
                if i > 0 and (i - 1, j) not in visited and grid[i - 1][j] != -1:
                    stack.append([i - 1, j, 0])
                    visited.add((i - 1, j))
            elif direct == 1:
                stack[-1][2] += 1
                if i < m - 1 and (i + 1, j) not in visited and grid[i + 1][j] != -1:
                    stack.append([i + 1, j, 0])
                    visited.add((i + 1, j))
            elif direct == 2:
                stack[-1][2] += 1
                if j > 0 and (i, j - 1) not in visited and grid[i][j - 1] != -1:
                    stack.append([i, j - 1, 0])
                    visited.add((i, j - 1))
            elif direct == 3:
                stack[-1][2] += 1
                if j < n - 1 and (i, j + 1) not in visited and grid[i][j + 1] != -1:
                    stack.append([i, j + 1, 0])
                    visited.add((i, j + 1))
            else:
                stack.pop()
                visited.remove((i, j))

        return ret
