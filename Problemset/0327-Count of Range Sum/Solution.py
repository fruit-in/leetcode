from sortedcontainers import SortedList


class Solution:
    def countRangeSum(self, nums: List[int], lower: int, upper: int) -> int:
        prefixsums = SortedList([0])
        prefixsum = 0
        ret = 0

        for i in range(len(nums)):
            prefixsum += nums[i]
            j = prefixsums.bisect_right(prefixsum - lower)
            k = prefixsums.bisect_left(prefixsum - upper)
            ret += j - k
            prefixsums.add(prefixsum)

        return ret
