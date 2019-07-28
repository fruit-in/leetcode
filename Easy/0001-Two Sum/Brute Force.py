class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for k, v in enumerate(nums):
            if target - v in nums[k + 1:]:
                return [k, k + 1 + nums[k + 1:].index(target - v)]
