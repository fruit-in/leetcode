from sortedcontainers import SortedList


class Solution:
    def medianSlidingWindow(self, nums: List[int], k: int) -> List[float]:
        window = SortedList(nums[:k])
        ret = [(window[(k - 1) // 2] + window[k // 2]) / 2]

        for i in range(len(nums) - k):
            window.remove(nums[i])
            window.add(nums[i + k])
            ret.append((window[(k - 1) // 2] + window[k // 2]) / 2)

        return ret
