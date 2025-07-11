class Solution:
    def minimumFuelCost(self, roads: List[List[int]], seats: int) -> int:
        def dfs(i: int, parent: int = -1) -> (int, int):
            fuel = 0
            representatives = 1

            for j in children[i]:
                if j != parent:
                    f, r = dfs(j, i)
                    fuel += f
                    representatives += r

            if i > 0:
                fuel += (representatives + seats - 1) // seats

            return (fuel, representatives)

        n = len(roads) + 1
        children = [[] for _ in range(n)]

        for a, b in roads:
            children[a].append(b)
            children[b].append(a)

        return dfs(0)[0]
