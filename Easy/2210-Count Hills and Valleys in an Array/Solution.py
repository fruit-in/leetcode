class Solution:
    def countHillValley(self, nums: List[int]) -> int:
        distinct_nums = [nums[0]]

        for num in nums[1:]:
            if num != distinct_nums[-1]:
                distinct_nums.append(num)

        return sum(1 for i in range(1, len(distinct_nums) - 1)
                   if (distinct_nums[i] > distinct_nums[i - 1]) == (distinct_nums[i] > distinct_nums[i + 1]))
