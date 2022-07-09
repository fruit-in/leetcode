class Solution:
    def validPath(self, n: int, edges: List[List[int]], source: int, destination: int) -> bool:
        paths = {}
        nodes = [source]
        seen = {source}

        for u, v in edges:
            if u not in paths:
                paths[u] = []
            if v not in paths:
                paths[v] = []
            paths[u].append(v)
            paths[v].append(u)

        while nodes:
            node0 = nodes.pop()
            for node1 in paths.get(node0, [node0]):
                if node1 == destination:
                    return True

                if node1 not in seen:
                    nodes.append(node1)
                    seen.add(node1)

        return False
