class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        s = {}
        for k, v in enumerate(nums):
            s[v] = k
        for k, v in enumerate(nums):
            if target - v in s.keys() and s[target - v] != k:
                return [k, s[target - v]]
