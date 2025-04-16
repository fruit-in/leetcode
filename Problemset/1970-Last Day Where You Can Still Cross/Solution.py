class Solution:
    def latestDayToCross(self, row: int, col: int, cells: List[List[int]]) -> int:
        def cannotCross(day: int) -> bool:
            grid = [[0] * col for _ in range(row)]
            stack = []
            visited = set()

            for r, c in cells[:day]:
                grid[r - 1][c - 1] = 1

            stack.extend((0, c) for c in range(col) if grid[0][c] == 0)
            visited.update((0, c) for c in range(col) if grid[0][c] == 0)

            while stack != []:
                r, c = stack.pop()

                if r == row - 1:
                    return False

                if r - 1 >= 0 and grid[r - 1][c] == 0 and (r - 1, c) not in visited:
                    stack.append((r - 1, c))
                    visited.add((r - 1, c))
                if r + 1 < row and grid[r + 1][c] == 0 and (r + 1, c) not in visited:
                    stack.append((r + 1, c))
                    visited.add((r + 1, c))
                if c - 1 >= 0 and grid[r][c - 1] == 0 and (r, c - 1) not in visited:
                    stack.append((r, c - 1))
                    visited.add((r, c - 1))
                if c + 1 < col and grid[r][c + 1] == 0 and (r, c + 1) not in visited:
                    stack.append((r, c + 1))
                    visited.add((r, c + 1))

            return True

        return bisect.bisect_left(list(range(len(cells))), True, key=cannotCross) - 1
