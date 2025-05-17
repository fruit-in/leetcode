class Solution:
    def beautifulSubsets(self, nums: List[int], k: int) -> int:
        def dfs(i: int) -> None:
            nonlocal ret
            if i == len(nums):
                return

            j = bisect.bisect_left(subset, nums[i] - k)
            if j == len(subset) or subset[j] != nums[i] - k:
                subset.append(nums[i])
                ret += 1
                dfs(i + 1)
                subset.pop()
            dfs(i + 1)

        subset = []
        ret = 0
        nums.sort()

        dfs(0)

        return ret
