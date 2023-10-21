class Solution:
    def numSimilarGroups(self, strs: List[str]) -> int:
        m = len(strs)
        n = len(strs[0])
        parent = list(range(m))

        for i in range(m):
            for j in range(i + 1, m):
                count = 0

                for k in range(n):
                    if strs[i][k] != strs[j][k]:
                        count += 1

                if count <= 2:
                    while parent[i] != parent[parent[i]]:
                        parent[i] = parent[parent[i]]
                    while parent[j] != parent[parent[j]]:
                        parent[j] = parent[parent[j]]
                    if parent[i] < parent[j]:
                        parent[parent[j]] = parent[i]
                    else:
                        parent[parent[i]] = parent[j]

        for i in range(m):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

        return sum(1 for i in range(m) if parent[i] == i)
