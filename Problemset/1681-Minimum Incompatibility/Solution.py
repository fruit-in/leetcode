class Solution:
    def minimumIncompatibility(self, nums: List[int], k: int) -> int:
        if max(Counter(nums).values()) > k:
            return -1
        if k == len(nums):
            return 0
        if k == 1:
            return max(nums) - min(nums)

        def dfs(i: int) -> None:
            nonlocal incompatibilities, minincompatibilities

            if subsets[-1] != [] and incompatibilities >= minincompatibilities:
                return

            if i == len(nums):
                minincompatibilities = incompatibilities
                return

            for j in range(k):
                if len(subsets[j]) == size or (subsets[j] != [] and subsets[j][-1] == nums[i]) or (j > 0 and subsets[j] == subsets[j - 1]):
                    continue

                subsets[j].append(nums[i])
                if len(subsets[j]) == 1:
                    incompatibilities -= nums[i]
                elif len(subsets[j]) == size:
                    incompatibilities += nums[i]
                dfs(i + 1)
                subsets[j].pop()
                if len(subsets[j]) == 0:
                    incompatibilities += nums[i]
                elif len(subsets[j]) == size - 1:
                    incompatibilities -= nums[i]

        size = len(nums) // k
        subsets = [[] for _ in range(k)]
        incompatibilities = 0
        minincompatibilities = inf
        nums.sort()
        dfs(0)

        return minincompatibilities
