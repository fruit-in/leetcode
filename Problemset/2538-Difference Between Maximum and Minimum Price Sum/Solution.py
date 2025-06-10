class Solution:
    def maxOutput(self, n: int, edges: List[List[int]], price: List[int]) -> int:
        def removeParentFromChildren(node: int, parent: int) -> None:
            i = children[node].index(parent)
            children[node][i], children[node][-1] = children[node][-1], children[node][i]
            children[node].pop()
            for child in children[node]:
                removeParentFromChildren(child, node)

        def setTop2(node: int) -> None:
            a, c0, b, c1 = -1, 0, -1, 0
            for child in children[node]:
                setTop2(child)
                c2 = top2[child][1] + price[child]
                if c2 >= c0:
                    a, c0, b, c1 = child, c2, a, c0
                elif c2 > c1:
                    b, c1 = child, c2
            top2[node] = [a, c0, b, c1]

        def maxCost(node: int, p: int) -> int:
            a, c0, _, c1 = top2[node]
            cost = max(c0, p)
            for child in children[node]:
                if child != a:
                    cost = max(cost, maxCost(child, max(p, c0) + price[node]))
                else:
                    cost = max(cost, maxCost(child, max(p, c1) + price[node]))
            return cost

        children = [[] for _ in range(n)]
        children[0].append(-1)
        top2 = [[] for _ in range(n)]

        for a, b in edges:
            children[a].append(b)
            children[b].append(a)

        removeParentFromChildren(0, -1)
        setTop2(0)

        return maxCost(0, 0)
