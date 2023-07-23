class Solution:
    def minAreaRect(self, points: List[List[int]]) -> int:
        points_set = set()
        min_area = None
        points.sort()

        for i in range(len(points)):
            x0, y0 = points[i]

            for j in range(i):
                x1, y1 = points[j]

                if (x0, y1) in points_set and (x1, y0) in points_set:
                    area = (x0 - x1) * abs(y0 - y1)
                    min_area = area if min_area is None \
                        else min(min_area, area)

            points_set.add((x0, y0))

        return 0 if min_area is None else min_area
