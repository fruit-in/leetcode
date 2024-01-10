import heapq


class Solution:
    def halveArray(self, nums: List[int]) -> int:
        currsum = sum(nums)
        halfsum = currsum / 2
        nums = [-num for num in nums]
        heapq.heapify(nums)
        ret = 0

        while currsum > halfsum:
            maxnum = -heapq.heappop(nums)
            currsum -= maxnum / 2
            heapq.heappush(nums, -maxnum / 2)
            ret += 1

        return ret
