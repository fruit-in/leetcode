from math import gcd


class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        ret = 1

        for i in range(len(points)):
            count = {}

            for j in range(i + 1, len(points)):
                dx = points[i][0] - points[j][0]
                dy = points[i][1] - points[j][1]

                if dx == 0:
                    k = None
                else:
                    neg = dx * dy < 0
                    dx = abs(dx)
                    dy = abs(dy)
                    g = gcd(dx, dy)
                    k = (neg, dy // g, dx // g)

                if k not in count:
                    count[k] = 1
                count[k] += 1
                ret = max(ret, count[k])

        return ret
