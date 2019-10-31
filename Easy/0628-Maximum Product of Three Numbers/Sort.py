class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        nums.sort()
        return nums[-1] * max(nums[-2] * nums[-3], nums[0] * nums[1])
