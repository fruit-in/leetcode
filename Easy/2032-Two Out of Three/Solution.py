class Solution:
    def twoOutOfThree(self, nums1: List[int], nums2: List[int], nums3: List[int]) -> List[int]:
        nums1 = set(nums1)
        nums2 = set(nums2)
        nums3 = set(nums3)
        nums12 = nums1.intersection(nums2)
        nums13 = nums1.intersection(nums3)
        nums23 = nums2.intersection(nums3)

        return list(nums12.union(nums13).union(nums23))
