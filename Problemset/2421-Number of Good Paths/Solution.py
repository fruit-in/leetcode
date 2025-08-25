class Solution:
    def numberOfGoodPaths(self, vals: List[int], edges: List[List[int]]) -> int:
        n = len(vals)
        parent = list(range(n))
        neighbors = [[] for _ in range(n)]
        valscount = Counter(vals)
        ret = sum(1 for c in valscount.values() if c == 1)

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for i in sorted(range(n), key=lambda i: vals[i]):
            if parent[i] != i or valscount[vals[i]] == 1:
                continue

            sameval = 1
            visited = {i}
            greater = set()

            while neighbors[i]:
                j = neighbors[i].pop()
                while parent[j] != parent[parent[j]]:
                    parent[j] = parent[parent[j]]
                j = parent[j]

                if vals[j] == vals[i]:
                    sameval += 1
                if vals[j] <= vals[i] and parent[j] != i:
                    parent[j] = i
                    for k in neighbors[j]:
                        while parent[k] != parent[parent[k]]:
                            parent[k] = parent[parent[k]]
                        k = parent[k]
                        if k not in visited:
                            neighbors[i].append(k)
                            visited.add(k)
                elif vals[j] > vals[i]:
                    greater.add(j)

            neighbors[i] = list(greater)
            ret += sameval * (sameval + 1) // 2

        return ret
