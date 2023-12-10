from math import gcd


class Solution:
    def minOperations(self, nums: List[int], numsDivide: List[int]) -> int:
        y = gcd(*numsDivide)

        for i, x in enumerate(sorted(nums)):
            if y % x == 0:
                return i
            if y < x:
                break

        return -1
