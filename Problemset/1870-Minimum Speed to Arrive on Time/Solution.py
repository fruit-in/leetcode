from math import ceil


class Solution:
    def minSpeedOnTime(self, dist: List[int], hour: float) -> int:
        if len(dist) - 0.999 > hour:
            return -1

        lo = 1
        hi = 10000000

        while lo < hi:
            v = (lo + hi) // 2
            t = sum(ceil(d / v) for d in dist[:-1]) + dist[-1] / v

            if t > hour:
                lo = v + 1
            else:
                hi = v

        return hi
