class Solution:
    def maximumCount(self, nums: List[int]) -> int:
        pos = bisect.bisect_left(nums, 0)
        neg = len(nums) - bisect.bisect_right(nums, 0)

        return max(pos, neg)
