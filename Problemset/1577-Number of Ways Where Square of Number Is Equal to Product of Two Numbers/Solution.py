class Solution:
    def numTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        count1 = Counter(nums1)
        count2 = Counter(nums2)
        ret = 0

        for x in nums1:
            x2 = x * x
            for y in nums2:
                if x2 % y == 0 and x2 // y in count2:
                    ret += count2[x2 // y]
                    if y == x:
                        ret -= 1
        for x in nums2:
            x2 = x * x
            for y in nums1:
                if x2 % y == 0 and x2 // y in count1:
                    ret += count1[x2 // y]
                    if y == x:
                        ret -= 1

        return ret // 2
