class Solution:
    def areConnected(self, n: int, threshold: int, queries: List[List[int]]) -> List[bool]:
        if threshold == 0:
            return [True] * len(queries)

        parent = list(range(n + 1))

        for x in range(threshold + 1, n + 1):
            for z in range(2, int(sqrt(x)) + 1):
                if x // z <= threshold:
                    break
                if x % z == 0:
                    if z > threshold:
                        while parent[z] != parent[parent[z]]:
                            parent[z] = parent[parent[z]]
                        parent[parent[z]] = x
                    z = x // z
                    while parent[z] != parent[parent[z]]:
                        parent[z] = parent[parent[z]]
                    parent[parent[z]] = x

        for x in range(threshold + 1, n + 1):
            while parent[x] != parent[parent[x]]:
                parent[x] = parent[parent[x]]

        return [parent[a] == parent[b] for a, b in queries]
