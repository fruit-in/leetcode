class Solution:
    def splitArraySameAverage(self, nums: List[int]) -> bool:
        if len(nums) < 2:
            return False

        s = sum(nums)
        sums = [set() for _ in range(len(nums) // 2 + 1)]
        sums[0].add(0)

        for num in nums:
            for i in range(len(sums) - 2, -1, -1):
                for x in sums[i]:
                    if (x + num) * len(nums) == s * (i + 1):
                        return True

                    sums[i + 1].add(x + num)

        return False
