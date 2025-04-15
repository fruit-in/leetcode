class Solution:
    def smallestDistancePair(self, nums: List[int], k: int) -> int:
        nums.sort()

        lo = 0
        hi = nums[-1] - nums[0]

        while lo < hi:
            mid = (lo + hi) // 2
            count = 0

            for i in range(len(nums)):
                j = bisect.bisect(nums, nums[i] + mid, lo=i + 1)
                count += j - i - 1

            if count < k:
                lo = mid + 1
            else:
                hi = mid

        return hi
