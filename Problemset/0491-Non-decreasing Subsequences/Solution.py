class Solution:
    def findSubsequences(self, nums: List[int]) -> List[List[int]]:
        def dfs(i: int) -> None:
            if i == len(nums):
                if len(sub) >= 2:
                    subsequences.add(tuple(sub))
                return

            if not sub or nums[i] >= sub[-1]:
                sub.append(nums[i])
                dfs(i + 1)
                sub.pop()
            dfs(i + 1)

        sub = []
        subsequences = set()
        dfs(0)

        return [list(sub) for sub in subsequences]
