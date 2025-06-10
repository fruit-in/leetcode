class Solution:
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        def removeParentFromChildren(node: int, parent: int) -> None:
            i = children[node].index(parent)
            children[node][i], children[node][-1] = children[node][-1], children[node][i]
            children[node].pop()
            for child in children[node]:
                removeParentFromChildren(child, node)

        def setHeight(node: int) -> None:
            for child in children[node]:
                setHeight(child)
                heights[node] = max(heights[node], heights[child] + 1)

        def findMinHeight(node: int, p: int) -> None:
            minheights[node] = max(heights[node], p)
            if len(children[node]) > 1:
                top2 = [children[node][0], children[node][1]]
                if heights[top2[0]] < heights[top2[1]]:
                    top2[0], top2[1] = top2[1], top2[0]
                for child in children[node][2:]:
                    if heights[child] >= heights[top2[0]]:
                        top2 = [child, top2[0]]
                    elif heights[child] >= heights[top2[1]]:
                        top2[1] = child
                for child in children[node]:
                    if child != top2[0]:
                        findMinHeight(child, max(p + 1, heights[top2[0]] + 2))
                    else:
                        findMinHeight(child, max(p + 1, heights[top2[1]] + 2))
            elif len(children[node]) == 1:
                findMinHeight(children[node][0], p + 1)

        children = [[] for _ in range(n)]
        children[0].append(-1)
        heights = [0] * n
        minheights = [0] * n

        for a, b in edges:
            children[a].append(b)
            children[b].append(a)

        removeParentFromChildren(0, -1)
        setHeight(0)
        findMinHeight(0, 0)
        minheight = min(minheights)

        return [node for node in range(n) if minheights[node] == minheight]
