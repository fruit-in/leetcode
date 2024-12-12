class Solution:
    def getAncestors(self, n: int, edges: List[List[int]]) -> List[List[int]]:
        indegrees = [0] * n
        ancestors = [set() for _ in range(n)]
        children = [[] for _ in range(n)]
        nodes = []
        answer = [[] for _ in range(n)]

        for u, v in edges:
            indegrees[v] += 1
            children[u].append(v)

        for i in range(n):
            if indegrees[i] == 0:
                nodes.append(i)

        while nodes != []:
            i = nodes.pop()
            answer[i] = sorted(ancestors[i])

            for j in children[i]:
                indegrees[j] -= 1
                ancestors[j].add(i)
                ancestors[j] |= ancestors[i]
                if indegrees[j] == 0:
                    nodes.append(j)

        return answer
