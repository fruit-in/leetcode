from functools import cache


class Solution:
    def predictTheWinner(self, nums: List[int]) -> bool:
        @cache
        def subArrayMaxDiff(i: int, j: int) -> int:
            if i == j:
                return nums[i]

            return max(nums[i] - subArrayMaxDiff(i + 1, j), nums[j] - subArrayMaxDiff(i, j - 1))

        return subArrayMaxDiff(0, len(nums) - 1) >= 0
