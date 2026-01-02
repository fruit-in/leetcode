class Solution:
    def maxNumEdgesToRemove(self, n: int, edges: List[List[int]]) -> int:
        parenta = list(range(n + 1))
        parentb = list(range(n + 1))
        counta = 0
        countb = 0
        ret = len(edges)

        edges.sort(key=lambda edge: -edge[0])

        for t, u, v in edges:
            while t != 2 and parenta[u] != parenta[parenta[u]]:
                parenta[u] = parenta[parenta[u]]
            while t != 2 and parenta[v] != parenta[parenta[v]]:
                parenta[v] = parenta[parenta[v]]
            while t != 1 and parentb[u] != parentb[parentb[u]]:
                parentb[u] = parentb[parentb[u]]
            while t != 1 and parentb[v] != parentb[parentb[v]]:
                parentb[v] = parentb[parentb[v]]

            if t == 3 and parenta[u] != parenta[v]:
                parenta[parenta[u]] = parenta[v]
                parentb[parentb[u]] = parentb[v]
                counta += 1
                countb += 1
                ret -= 1
            elif t == 2 and parentb[u] != parentb[v]:
                parentb[parentb[u]] = parentb[v]
                countb += 1
                ret -= 1
            elif t == 1 and parenta[u] != parenta[v]:
                parenta[parenta[u]] = parenta[v]
                counta += 1
                ret -= 1

        return ret if counta == countb == n - 1 else -1
