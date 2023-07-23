class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        min1, min2 = 1000, 1000
        max1, max2, max3 = -1000, -1000, -1000

        for n in nums:
            if n <= min1:
                min2 = min1
                min1 = n
            elif n < min2:
                min2 = n

            if n >= max3:
                max1 = max2
                max2 = max3
                max3 = n
            elif n >= max2:
                max1 = max2
                max2 = n
            elif n > max1:
                max1 = n

        return max3 * max(min1 * min2, max1 * max2)
