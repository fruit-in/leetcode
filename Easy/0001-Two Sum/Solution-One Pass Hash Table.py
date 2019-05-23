class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        s = {}
        for k, v in enumerate(nums):
            if target - v in s.keys():
                return [k, s[target - v]]
            s[v] = k
