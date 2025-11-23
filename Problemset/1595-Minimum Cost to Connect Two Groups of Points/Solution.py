class Solution:
    def connectTwoGroups(self, cost: List[List[int]]) -> int:
        @cache
        def dfs(mask: int, i: int, j: int) -> int:
            if i == size1:
                return 0 if mask == (1 << size2) - 1 else inf
            if j == size2:
                return inf

            ret = dfs(mask, i, j + 1)
            mask |= 1 << j
            ret = min(ret, cost[i][j] +
                      min(dfs(mask, i + 1, 0), dfs(mask, i, j + 1)))

            return ret

        size1 = len(cost)
        size2 = len(cost[0])

        return dfs(0, 0, 0)
