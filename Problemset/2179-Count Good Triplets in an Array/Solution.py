from sortedcontainers import SortedList


class Solution:
    def goodTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        pos1 = {x: i for i, x in enumerate(nums1)}
        pos1x = SortedList()
        pos1z = SortedList()
        count = [0] * len(nums2)
        ret = 0

        for i in range(len(nums2)):
            count[i] = pos1x.bisect_left(pos1[nums2[i]])
            pos1x.add(pos1[nums2[i]])

        for i in range(len(nums2) - 1, -1, -1):
            ret += count[i] * (len(pos1z) - pos1z.bisect_left(pos1[nums2[i]]))
            pos1z.add(pos1[nums2[i]])

        return ret
