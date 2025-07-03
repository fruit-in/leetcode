class TreeAncestor:

    def __init__(self, n: int, parent: List[int]):
        def dfs(i: int):
            while 1 << len(self.ancestors[i]) <= self.depth[i]:
                self.ancestors[i].append(self.getKthAncestor(
                    parent[i], ((1 << len(self.ancestors[i])) - 1)))
            for child in children[i]:
                self.depth[child] = self.depth[i] + 1
                dfs(child)

        children = [[] for _ in range(n)]
        self.depth = [0] * n
        self.ancestors = [[] for _ in range(n)]
        for i in range(1, len(parent)):
            children[parent[i]].append(i)
        dfs(0)

    def getKthAncestor(self, node: int, k: int) -> int:
        if k == 0:
            return node
        if k > self.depth[node]:
            return -1

        return self.getKthAncestor(self.ancestors[node][k.bit_length() - 1], k ^ (1 << (k.bit_length() - 1)))


# Your TreeAncestor object will be instantiated and called as such:
# obj = TreeAncestor(n, parent)
# param_1 = obj.getKthAncestor(node,k)
