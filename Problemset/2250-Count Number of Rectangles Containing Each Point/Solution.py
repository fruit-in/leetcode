class Solution:
    def countRectangles(self, rectangles: List[List[int]], points: List[List[int]]) -> List[int]:
        sortedls = [[] for _ in range(max(h for _, h in rectangles) + 1)]
        count = [0] * len(points)

        for l, h in rectangles:
            sortedls[h].append(l)

        for i in range(len(sortedls)):
            sortedls[i].sort()

        for i, [x, y] in enumerate(points):
            for h in range(y, len(sortedls)):
                count[i] += len(sortedls[h]) - bisect_left(sortedls[h], x)

        return count
