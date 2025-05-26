class Solution:
    def minimumXORSum(self, nums1: List[int], nums2: List[int]) -> int:
        @cache
        def dfs(i: int, mask: int) -> int:
            if i == len(nums1):
                return 0

            ret = inf

            for j in range(len(nums2)):
                if (mask >> j) & 1 == 0:
                    ret = min(ret, (nums1[i] ^ nums2[j]) +
                              dfs(i + 1, mask | (1 << j)))

            return ret

        return dfs(0, 0)
