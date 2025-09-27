class Solution:
    def findCriticalAndPseudoCriticalEdges(self, n: int, edges: List[List[int]]) -> List[List[int]]:
        def getMSTWeight(initial: (int, int, int, int), delete: int = -1) -> int:
            if initial[0] == delete:
                initial = edges[1]

            parent = list(range(n))
            parent[initial[1]] = initial[2]
            total = initial[3]
            edgecount = 1

            for i, a, b, w in edges:
                if i == initial[0] or i == delete:
                    continue

                while parent[a] != parent[parent[a]]:
                    parent[a] = parent[parent[a]]
                while parent[b] != parent[parent[b]]:
                    parent[b] = parent[parent[b]]

                if parent[a] != parent[b]:
                    parent[parent[a]] = parent[b]
                    total += w
                    edgecount += 1

            return total if edgecount == n - 1 else inf

        if n == 2:
            return [[0], []]

        edges = [(i, a, b, w) for i, [a, b, w] in enumerate(edges)]
        edges.sort(key=lambda edge: edge[3])
        minweight = getMSTWeight(edges[0])
        ret = [[], []]

        for edge in edges:
            if getMSTWeight(edges[0], edge[0]) > minweight:
                ret[0].append(edge[0])
            elif getMSTWeight(edge) == minweight:
                ret[1].append(edge[0])

        return ret
