class Solution:
    def numberOfArithmeticSlices(self, nums: List[int]) -> int:
        count = {}
        ret = 0

        for i in range(1, len(nums)):
            for j in range(i):
                d = nums[i] - nums[j]
                c = count.get((j, d), 0)
                count[(i, d)] = count.get((i, d), 0) + c + 1
                ret += c

        return ret
