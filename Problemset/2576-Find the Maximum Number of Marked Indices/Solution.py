class Solution:
    def maxNumOfMarkedIndices(self, nums: List[int]) -> int:
        n = len(nums)
        lo = n // 2
        nums.sort()

        for i in range(n // 2):
            lo = bisect_left(nums, 2 * nums[i], lo=lo) + 1
            if lo > n:
                return i * 2

        return n // 2 * 2
