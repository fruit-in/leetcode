class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        nums.sort()
        ret = 0

        for i in range(len(nums) - 1):
            if nums[i] + nums[-1] < lower:
                continue
            if nums[i] + nums[i + 1] > upper:
                break

            j = max(bisect.bisect_left(nums, lower - nums[i]), i + 1)
            k = bisect.bisect(nums, upper - nums[i])
            ret += k - j

        return ret
