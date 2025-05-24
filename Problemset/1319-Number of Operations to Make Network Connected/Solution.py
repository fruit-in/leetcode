class Solution:
    def makeConnected(self, n: int, connections: List[List[int]]) -> int:
        parent = list(range(n))
        groupnodes = {i: 1 for i in range(n)}
        groupcables = {i: 0 for i in range(n)}

        for a, b in connections:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] != parent[b]:
                groupnodes[parent[a]] += groupnodes.pop(parent[b])
                groupcables[parent[a]] += groupcables.pop(parent[b])
                parent[parent[b]] = parent[a]
            groupcables[parent[a]] += 1

        redundant = sum(groupcables[i] - groupnodes[i] + 1 for i in groupnodes)
        needed = len(groupnodes) - 1

        if redundant < needed:
            return -1

        return needed
