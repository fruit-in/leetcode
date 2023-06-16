class Solution:
    def reachableNodes(self, n: int, edges: List[List[int]], restricted: List[int]) -> int:
        restricted = set(restricted)
        edgesmap = {}
        visited = {0}
        nodes = [0]

        for a, b in edges:
            if a not in edgesmap:
                edgesmap[a] = []
            if b not in edgesmap:
                edgesmap[b] = []
            edgesmap[a].append(b)
            edgesmap[b].append(a)

        while nodes:
            curr = nodes.pop()
            for node in edgesmap[curr]:
                if node not in restricted and node not in visited:
                    visited.add(node)
                    nodes.append(node)

        return len(visited)
