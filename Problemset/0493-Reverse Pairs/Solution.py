from sortedcontainers import SortedList


class Solution:
    def reversePairs(self, nums: List[int]) -> int:
        sortednums = SortedList()
        ret = 0

        for num in nums:
            ret += len(sortednums) - sortednums.bisect_right(2 * num)
            sortednums.add(num)

        return ret
