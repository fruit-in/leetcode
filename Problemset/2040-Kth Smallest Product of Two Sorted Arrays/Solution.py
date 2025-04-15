import math


class Solution:
    def kthSmallestProduct(self, nums1: List[int], nums2: List[int], k: int) -> int:
        if len(nums2) < len(nums1):
            nums1, nums2 = nums2, nums1

        lo = min(nums1[0] * nums2[0], nums1[0] * nums2[-1],
                 nums1[-1] * nums2[0], nums1[-1] * nums2[-1])
        hi = max(nums1[0] * nums2[0], nums1[0] * nums2[-1],
                 nums1[-1] * nums2[0], nums1[-1] * nums2[-1])

        while lo < hi:
            mid = (lo + hi) // 2
            count = 0

            for i in range(len(nums1)):
                if nums1[i] == 0:
                    count += len(nums2) if mid >= 0 else 0
                elif nums1[i] > 0:
                    count += bisect.bisect(nums2, mid // nums1[i])
                else:
                    count += len(nums2) - bisect.bisect(nums2,
                                                        math.ceil(mid / nums1[i]) - 1)

            if count < k:
                lo = mid + 1
            else:
                hi = mid

        return hi
