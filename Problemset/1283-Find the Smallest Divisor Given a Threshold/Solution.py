import bisect
import math


class Solution:
    def smallestDivisor(self, nums: List[int], threshold: int) -> int:
        divisors = list(range(1, max(nums) + 1))

        return 1 + bisect.bisect_left(
            divisors,
            True,
            key=lambda x: sum(math.ceil(num / x) for num in nums) <= threshold
        )
