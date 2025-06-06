class Solution:
    def isPrintable(self, targetGrid: List[List[int]]) -> bool:
        m, n = len(targetGrid), len(targetGrid[0])
        position = {targetGrid[r][c]: [r, r, c, c]
                    for r in range(m) for c in range(n)}
        colorgrid = {color: set() for color in position}
        belongto = [[set() for _ in range(n)] for _ in range(m)]
        count0 = {color: 0 for color in position}
        stack = []

        for r in range(m):
            for c, color in enumerate(targetGrid[r]):
                position[color][0] = min(position[color][0], r)
                position[color][1] = max(position[color][1], r)
                position[color][2] = min(position[color][2], c)
                position[color][3] = max(position[color][3], c)
                colorgrid[color].add((r, c))

        for r in range(m):
            for c in range(n):
                for color, [top, bottom, left, right] in position.items():
                    if top <= r <= bottom and left <= c <= right:
                        belongto[r][c].add(color)

        for color, [top, bottom, left, right] in position.items():
            if (bottom - top + 1) * (right - left + 1) == len(colorgrid[color]):
                stack.append(color)

        while stack:
            for r, c in colorgrid[stack.pop()]:
                targetGrid[r][c] = 0

                for color in belongto[r][c]:
                    top, bottom, left, right = position[color]
                    count0[color] += 1
                    if count0[color] + len(colorgrid[color]) == (bottom - top + 1) * (right - left + 1):
                        stack.append(color)

        return all(targetGrid[r][c] == 0 for r in range(m) for c in range(n))
