class Solution:
    def maximalPathQuality(self, values: List[int], edges: List[List[int]], maxTime: int) -> int:
        def dfs(u: int, time: int) -> None:
            nonlocal quality, ret
            if u == 0:
                ret = max(ret, quality)
            if usetimes[u] == 0:
                quality += values[u]
            usetimes[u] += 1

            for v, t in neighbors[u].items():
                if time + t <= maxTime:
                    dfs(v, time + t)

            usetimes[u] -= 1
            if usetimes[u] == 0:
                quality -= values[u]

        n = len(values)
        neighbors = [{} for _ in range(n)]
        usetimes = [0] * n
        quality = 0
        ret = values[0]

        for u, v, time in edges:
            if time <= maxTime:
                neighbors[u][v] = time
                neighbors[v][u] = time

        dfs(0, 0)

        return ret
