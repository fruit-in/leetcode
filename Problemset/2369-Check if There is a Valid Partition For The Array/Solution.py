class Solution:
    def validPartition(self, nums: List[int]) -> bool:
        @cache
        def dfs(i: int) -> bool:
            return i == len(nums) or (i + 1 < len(nums) and nums[i] == nums[i + 1] and dfs(i + 2)) or (i + 2 < len(nums) and nums[i] == nums[i + 1] == nums[i + 2] and dfs(i + 3)) or (i + 2 < len(nums) and nums[i] == nums[i + 1] - 1 == nums[i + 2] - 2 and dfs(i + 3))

        return dfs(0)
