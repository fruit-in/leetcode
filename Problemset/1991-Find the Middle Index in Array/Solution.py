class Solution:
    def findMiddleIndex(self, nums: List[int]) -> int:
        total_sum = sum(nums)
        left_sum = 0

        for i in range(len(nums)):
            if left_sum * 2 + nums[i] == total_sum:
                return i
            left_sum += nums[i]

        return -1
