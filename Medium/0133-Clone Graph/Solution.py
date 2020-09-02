"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        if not node:
            return None

        nodes = [node]
        visited = [False] * 101
        visited[node.val] = True
        i = 0

        while i < len(nodes):
            node = nodes[i]

            for neighbor in node.neighbors:
                if not visited[neighbor.val]:
                    visited[neighbor.val] = True
                    nodes.append(neighbor)
            node.neighbors.append(Node(node.val))

            i += 1

        for node in nodes:
            copy = node.neighbors[-1]
            for neighbor in node.neighbors[:-1]:
                copy.neighbors.append(neighbor.neighbors[-1])

        copy = nodes[0].neighbors[-1]

        for node in nodes:
            node.neighbors.pop()

        return copy
