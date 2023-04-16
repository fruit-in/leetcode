import bisect


class Solution:
    def maxDistance(self, nums1: List[int], nums2: List[int]) -> int:
        ret = 0

        for i in range(len(nums1)):
            j = bisect.bisect_right(
                nums2, False, key=lambda x: x < nums1[i]) - 1
            ret = max(ret, j - i)

        return ret
