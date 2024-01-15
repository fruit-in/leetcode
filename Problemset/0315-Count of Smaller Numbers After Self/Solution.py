from sortedcontainers import SortedList


class Solution:
    def countSmaller(self, nums: List[int]) -> List[int]:
        sortednums = SortedList()
        counts = [0] * len(nums)

        for i in range(len(nums) - 1, -1, -1):
            counts[i] = sortednums.bisect_left(nums[i])
            sortednums.add(nums[i])

        return counts
