class Solution:
    def sumOfFlooredPairs(self, nums: List[int]) -> int:
        prevsum = 0
        ret = 0

        nums.sort()

        for j in range(len(nums)):
            if j > 0 and nums[j] == nums[j - 1]:
                ret = (ret + prevsum) % 1000000007
                continue

            i = bisect.bisect(nums, nums[j] - 1)
            prevsum = 0

            for x in range(1, nums[-1] // nums[j] + 1):
                k = bisect.bisect(nums, nums[j] * (x + 1) - 1, lo=i)
                prevsum = (prevsum + x * (k - i)) % 1000000007
                i = k

            ret = (ret + prevsum) % 1000000007

        return ret
