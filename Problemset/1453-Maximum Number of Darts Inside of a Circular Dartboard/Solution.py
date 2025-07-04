class Solution:
    def numPoints(self, darts: List[List[int]], r: int) -> int:
        ret = 1

        for x1, y1 in darts:
            for x2, y2 in darts:
                base = dist((x1, y1), (x2, y2))
                if base == 0 or base > 2 * r:
                    continue
                h = sqrt(r ** 2 - base ** 2 / 4)
                x0, y0 = (x1 + x2) / 2 + h * (y1 - y2) / \
                    base, (y1 + y2) / 2 + h * (x2 - x1) / base
                ret = max(ret, sum(dist((x3, y3), (x0, y0)) -
                          r <= .00001 for x3, y3 in darts))

        return ret
