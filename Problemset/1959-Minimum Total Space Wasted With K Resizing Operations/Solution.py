class Solution:
    def minSpaceWastedKResizing(self, nums: List[int], k: int) -> int:
        @cache
        def dfs(i: int, k: int) -> int:
            if i == len(nums):
                return 0
            if k == 1:
                return max(nums[i:]) * (len(nums) - i) - sum(nums[i:])

            maxnum = 0
            numssum = 0
            ret = inf

            for j in range(i, len(nums)):
                maxnum = max(maxnum, nums[j])
                numssum += nums[j]
                ret = min(ret, maxnum * (j - i + 1) -
                          numssum + dfs(j + 1, k - 1))

            return ret

        return dfs(0, k + 1)
