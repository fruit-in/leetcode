class Solution:
    def minDistance(self, houses: List[int], k: int) -> int:
        @cache
        def dfs(i: int, k: int) -> int:
            if i == len(houses):
                return 0
            if k == 0:
                return inf

            ret = inf

            for j in range(i + 1, len(houses) + 1):
                mid = (i + j) // 2
                distl = houses[mid] * (mid - i) - prefixsum[mid] + prefixsum[i]
                distr = prefixsum[j] - prefixsum[mid] - houses[mid] * (j - mid)
                ret = min(ret, distl + distr + dfs(j, k - 1))

            return ret

        prefixsum = [0] * (len(houses) + 1)
        houses.sort()

        for i in range(len(houses)):
            prefixsum[i + 1] = prefixsum[i] + houses[i]

        return dfs(0, k)
