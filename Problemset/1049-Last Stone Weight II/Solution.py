class Solution:
    def lastStoneWeightII(self, stones: List[int]) -> int:
        @cache
        def dfs(i: int, target: int) -> int:
            if i == len(stones):
                return target

            return min(dfs(i + 1, abs(target - stones[i])), dfs(i + 1, abs(target + stones[i])))

        return dfs(0, 0)
