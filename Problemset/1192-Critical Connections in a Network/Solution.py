class Solution:
    def criticalConnections(self, n: int, connections: List[List[int]]) -> List[List[int]]:
        def tarjan(a: int, parent: int) -> None:
            nonlocal time
            disc[a] = time
            low[a] = time
            time += 1

            for b in neighbors[a]:
                if b == parent:
                    continue

                if disc[b] == -1:
                    tarjan(b, a)
                    low[a] = min(low[a], low[b])

                    if low[b] > disc[a]:
                        ret.append([a, b])
                else:
                    low[a] = min(low[a], disc[b])

        neighbors = [[] for _ in range(n)]
        time = 0
        disc = [-1] * n
        low = [-1] * n
        ret = []

        for a, b in connections:
            neighbors[a].append(b)
            neighbors[b].append(a)

        tarjan(0, -1)

        return ret
