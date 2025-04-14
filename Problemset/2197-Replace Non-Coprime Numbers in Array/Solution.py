from math import gcd, lcm


class Solution:
    def replaceNonCoprimes(self, nums: List[int]) -> List[int]:
        stack = []

        for x in nums:
            while stack != [] and gcd(stack[-1], x) > 1:
                x = lcm(stack.pop(), x)
            stack.append(x)

        return stack
