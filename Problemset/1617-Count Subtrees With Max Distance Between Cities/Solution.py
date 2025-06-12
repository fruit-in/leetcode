class Solution:
    def countSubgraphsForEachDiameter(self, n: int, edges: List[List[int]]) -> List[int]:
        def buildTree(x: int) -> (int, List[List[int]]):
            countnodes = x.bit_count()
            countedges = 0
            root = -1
            children = [[] for _ in range(n)]

            for u, v in edges:
                if (x >> u) & 1 == 1 and (x >> v) & 1 == 1:
                    countedges += 1
                    root = u
                    children[u].append(v)
                    children[v].append(u)

            if countnodes == 1 or countnodes - countedges != 1:
                return (-1, [])

            stack = [root]
            while stack:
                node = stack.pop()
                for child in children[node]:
                    children[child].remove(node)
                    stack.append(child)

            return (root, children)

        def maxDistance(root: int, children: List[List[int]]) -> (int, int):
            maxdist, maxheight = 0, 0
            c0, h0, c1, h1 = 0, -1, 0, -1

            for child in children[root]:
                dist, height = maxDistance(child, children)
                maxdist = max(maxdist, dist)
                maxheight = max(maxheight, height + 1)

                if height >= h0:
                    c0, h0, c1, h1 = child, height, c0, h0
                elif height >= h1:
                    c1, h1 = child, height

            return (max(maxdist, h0 + h1 + 2), maxheight)

        edges = [[u - 1, v - 1] for u, v in edges]
        ans = [0] * (n - 1)

        for x in range(3, 1 << n):
            root, children = buildTree(x)
            if root >= 0:
                ans[maxDistance(root, children)[0] - 1] += 1

        return ans
