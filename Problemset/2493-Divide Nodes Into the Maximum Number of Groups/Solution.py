class Solution:
    def magnificentSets(self, n: int, edges: List[List[int]]) -> int:
        parent = list(range(n + 1))
        neighbors = [[] for _ in range(n + 1)]
        unionfinds = {}
        ret = 0

        for a, b in edges:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            parent[parent[a]] = parent[b]
            neighbors[a].append(b)
            neighbors[b].append(a)

        for node in range(1, n + 1):
            while parent[node] != parent[parent[node]]:
                parent[node] = parent[parent[node]]

            if parent[node] not in unionfinds:
                unionfinds[parent[node]] = []
            unionfinds[parent[node]].append(node)

        for unionfind in unionfinds.values():
            m = 1

            for node in unionfind:
                used = {node: 1}
                queue = deque([node])

                while queue:
                    node = queue.popleft()
                    i = used[node]

                    for neighbor in neighbors[node]:
                        if neighbor not in used:
                            used[neighbor] = i + 1
                            queue.append(neighbor)
                            m = max(m, i + 1)
                        elif abs(used[neighbor] - i) != 1:
                            return -1

            ret += m

        return ret
