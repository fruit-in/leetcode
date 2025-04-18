# """
# This is MountainArray's API interface.
# You should not implement it, or speculate about its implementation
# """
# class MountainArray:
#    def get(self, index: int) -> int:
#    def length(self) -> int:

class Solution:
    def findInMountainArray(self, target: int, mountainArr: 'MountainArray') -> int:
        n = mountainArr.length()
        i = bisect.bisect(range(n - 1), False, key=lambda j: mountainArr.get(
            j) > mountainArr.get(j + 1))

        index = bisect.bisect_left(range(n), target, hi=i, key=mountainArr.get)
        if mountainArr.get(index) == target:
            return index
        index = bisect.bisect_left(
            range(n - 1), -target, lo=i, key=lambda j: -mountainArr.get(j))
        if mountainArr.get(index) == target:
            return index

        return -1
