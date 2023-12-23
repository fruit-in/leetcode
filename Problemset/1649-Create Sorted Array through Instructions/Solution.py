from sortedcontainers import SortedList


class Solution:
    def createSortedArray(self, instructions: List[int]) -> int:
        nums = SortedList()
        ret = 0

        for num in instructions:
            nums.add(num)
            less = nums.bisect_left(num)
            greater = len(nums) - nums.bisect_right(num)
            ret += min(less, greater)

        return ret % 1000000007
