class Solution:
    def lengthOfLIS(self, nums: List[int], k: int) -> int:
        size = 1 << ceil(log2(max(nums) + 1))
        tree = [0] * (2 * size)
        dp = [0] * len(nums)

        for i in range(len(nums)):
            left = max(nums[i] - k, 0) + size
            right = nums[i] - 1 + size
            while left <= right:
                if left % 2 == 1:
                    dp[i] = max(dp[i], tree[left] + 1)
                    left += 1
                if right % 2 == 0:
                    dp[i] = max(dp[i], tree[right] + 1)
                    right -= 1
                left >>= 1
                right >>= 1

            j = nums[i] + size
            tree[j] = dp[i]
            while j > 1:
                j >>= 1
                tree[j] = max(tree[2 * j], tree[2 * j + 1])

        return max(dp)
