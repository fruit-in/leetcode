class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        stack = []
        ret = 0

        for r0 in range(m):
            for c0 in range(n):
                if grid[r0][c0] == '1':
                    ret += 1
                    grid[r0][c0] = '0'
                    stack.append((r0, c0))
                    while stack:
                        r1, c1 = stack.pop()
                        for (i, j) in [(0, -1), (0, 1), (1, 0), (-1, 0)]:
                            r2, c2 = r1 + i, c1 + j
                            if 0 <= r2 < m and 0 <= c2 < n and grid[r2][c2] == '1':
                                grid[r2][c2] = '0'
                                stack.append((r2, c2))

        return ret
