class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        def dfs(i: int) -> bool:
            if i == len(nums):
                return True

            prevsets = set()

            for j in range(k):
                if subsets[j] not in prevsets and subsets[j] + nums[i] <= subsum:
                    prevsets.add(subsets[j])
                    subsets[j] += nums[i]
                    if dfs(i + 1):
                        return True
                    subsets[j] -= nums[i]

            return False

        if sum(nums) % k != 0:
            return False

        nums.sort()
        subsum = sum(nums) // k

        while nums and nums[-1] == subsum:
            nums.pop()
            k -= 1

        if k > 0 and (nums[-1] > subsum or nums[-1] + nums[0] > subsum):
            return False

        subsets = [0] * k
        nums.reverse()

        return dfs(0)
