class Solution:
    def minimumHammingDistance(self, source: List[int], target: List[int], allowedSwaps: List[List[int]]) -> int:
        n = len(source)
        parent = list(range(n))
        groups = {}
        ret = n

        for a, b in allowedSwaps:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(n):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

            if parent[i] not in groups:
                groups[parent[i]] = {}
            if source[i] not in groups[parent[i]]:
                groups[parent[i]][source[i]] = 0
            groups[parent[i]][source[i]] += 1

        for i in range(n):
            if groups[parent[i]].get(target[i], 0) > 0:
                groups[parent[i]][target[i]] -= 1
                ret -= 1

        return ret
