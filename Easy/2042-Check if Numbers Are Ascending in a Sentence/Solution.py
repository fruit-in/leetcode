class Solution:
    def areNumbersAscending(self, s: str) -> bool:
        nums = [int(token) for token in s.split() if token.isdecimal()]

        return all(nums[i - 1] < nums[i] for i in range(1, len(nums)))
