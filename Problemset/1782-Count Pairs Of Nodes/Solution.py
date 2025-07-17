class Solution:
    def countPairs(self, n: int, edges: List[List[int]], queries: List[int]) -> List[int]:
        degree = [0] * n
        multiplecount = {}
        answers = [0] * len(queries)

        for u, v in edges:
            u, v = min(u, v) - 1, max(u, v) - 1
            degree[u] += 1
            degree[v] += 1
            if (u, v) not in multiplecount:
                multiplecount[(u, v)] = 0
            multiplecount[(u, v)] += 1

        sorteddegree = sorted(degree)

        for i in range(len(queries)):
            for j in range(n):
                answers[i] += n - \
                    bisect_right(sorteddegree, queries[i] - degree[j])
                if degree[j] * 2 > queries[i]:
                    answers[i] -= 1
            answers[i] //= 2

            for (u, v), c in multiplecount.items():
                if degree[u] + degree[v] > queries[i] and degree[u] + degree[v] - c <= queries[i]:
                    answers[i] -= 1

        return answers
