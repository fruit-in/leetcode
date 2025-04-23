class Solution:
    def friendRequests(self, n: int, restrictions: List[List[int]], requests: List[List[int]]) -> List[bool]:
        parent = {p: p for p in range(n)}
        group = {p: {p} for p in range(n)}
        result = [True] * len(requests)

        for i, [u, v] in enumerate(requests):
            for x, y in restrictions:
                if (x in group[parent[u]] and y in group[parent[v]]) or (y in group[parent[u]] and x in group[parent[v]]):
                    result[i] = False
                    break

            if result[i] and parent[u] != parent[v]:
                if len(group[parent[u]]) < len(group[parent[v]]):
                    u, v = v, u
                for p in group.pop(parent[v]):
                    parent[p] = parent[u]
                    group[parent[u]].add(p)

        return result
