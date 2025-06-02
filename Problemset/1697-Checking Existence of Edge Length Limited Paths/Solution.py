class Solution:
    def distanceLimitedPathsExist(self, n: int, edgeList: List[List[int]], queries: List[List[int]]) -> List[bool]:
        parent = list(range(n))
        indices = sorted(range(len(queries)), key=lambda i: queries[i][2])
        i = 0
        answer = [False] * len(queries)
        edgeList.sort(key=lambda e: e[2])

        for j in indices:
            while i < len(edgeList) and edgeList[i][2] < queries[j][2]:
                u, v, _ = edgeList[i]
                while parent[u] != parent[parent[u]]:
                    parent[u] = parent[parent[u]]
                while parent[v] != parent[parent[v]]:
                    parent[v] = parent[parent[v]]
                parent[parent[u]] = parent[v]
                i += 1

            p, q, _ = queries[j]
            while parent[p] != parent[parent[p]]:
                parent[p] = parent[parent[p]]
            while parent[q] != parent[parent[q]]:
                parent[q] = parent[parent[q]]

            answer[j] = parent[p] == parent[q]

        return answer
