class Solution:
    def intersection(self, nums: List[List[int]]) -> List[int]:
        count = {}

        for array in nums:
            for num in array:
                if num not in count:
                    count[num] = 0
                count[num] += 1

        return sorted(k for k, v in count.items() if v == len(nums))
