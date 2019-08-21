class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if not nums or target <= nums[0]:
            return 0
        if target > nums[-1]:
            return len(nums)
        for i in range(1, len(nums)):
            if target == nums[i] or (target > nums[i - 1] and target < nums[i]):
                return i
