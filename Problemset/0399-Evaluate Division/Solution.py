class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        parent = {}
        variablevalue = {}
        ret = []

        for (a, b), v in zip(equations, values):
            if a not in parent:
                variablevalue[a] = 1.
                parent[a] = a
            if b not in parent:
                variablevalue[b] = 1.
                parent[b] = b

            while parent[a] != parent[parent[a]]:
                variablevalue[a] *= variablevalue[parent[a]]
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                variablevalue[b] *= variablevalue[parent[b]]
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                variablevalue[parent[b]] = variablevalue[a] / \
                    variablevalue[b] / v
                parent[parent[b]] = parent[a]
            else:
                variablevalue[parent[a]] = variablevalue[b] / \
                    variablevalue[a] * v
                parent[parent[a]] = parent[b]

        for a in parent:
            while parent[a] != parent[parent[a]]:
                variablevalue[a] *= variablevalue[parent[a]]
                parent[a] = parent[parent[a]]

        for c, d in queries:
            if c in parent and d in parent and parent[c] == parent[d]:
                ret.append(variablevalue[c] / variablevalue[d])
            else:
                ret.append(-1.)

        return ret
