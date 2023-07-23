class Solution:
    def sortEvenOdd(self, nums: List[int]) -> List[int]:
        odds = sorted(nums[i] for i in range(1, len(nums), 2))[::-1]
        evens = sorted(nums[i] for i in range(0, len(nums), 2))

        return [odds[i // 2] if i % 2 == 1 else evens[i // 2] for i in range(len(nums))]
