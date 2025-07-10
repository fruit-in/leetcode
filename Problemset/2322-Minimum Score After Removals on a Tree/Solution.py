class Solution:
    def minimumScore(self, nums: List[int], edges: List[List[int]]) -> int:
        def dfs(i: int) -> None:
            for j in children[i]:
                children[j].remove(i)
                ancestors[j] = ancestors[i] | {i}
                dfs(j)
                subtreexor[i] ^= subtreexor[j]

        n = len(nums)
        children = [set() for _ in range(n)]
        ancestors = [set() for _ in range(n)]
        subtreexor = nums.copy()
        ret = inf

        for a, b in edges:
            children[a].add(b)
            children[b].add(a)

        dfs(0)

        for edge in edges:
            if edge[0] in children[edge[1]]:
                edge[0], edge[1] = edge[1], edge[0]

        for i in range(n - 1):
            a, b = edges[i]
            for j in range(i + 1, n - 1):
                c, d = edges[j]
                xor1 = subtreexor[b]
                xor2 = subtreexor[d]
                if c == b or b in ancestors[c]:
                    xor1 ^= xor2
                elif a == d or d in ancestors[a]:
                    xor2 ^= xor1
                xor3 = subtreexor[0] ^ xor1 ^ xor2
                ret = min(ret, max(xor1, xor2, xor3) - min(xor1, xor2, xor3))

        return ret
