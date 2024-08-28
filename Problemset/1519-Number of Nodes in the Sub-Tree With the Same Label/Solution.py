class Solution:
    def countSubTrees(self, n: int, edges: List[List[int]], labels: str) -> List[int]:
        children = [set() for _ in range(n)]
        parent = [-1] * n
        stack = [0]
        count = [[0] * 26 for _ in range(n)]
        ans = [0] * n

        for (a, b) in edges:
            children[a].add(b)
            children[b].add(a)

        while stack != []:
            i = stack.pop()

            for j in children[i]:
                children[j].remove(i)
                parent[j] = i
                stack.append(j)

        stack = [i for i in range(n) if len(children[i]) == 0]

        while stack != []:
            i = stack.pop()

            count[i][ord(labels[i]) - 97] += 1
            ans[i] = count[i][ord(labels[i]) - 97]

            if parent[i] != -1:
                for j in range(26):
                    count[parent[i]][j] += count[i][j]
                children[parent[i]].remove(i)
                if len(children[parent[i]]) == 0:
                    stack.append(parent[i])

        return ans
