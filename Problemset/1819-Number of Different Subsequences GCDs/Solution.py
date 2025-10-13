class Solution:
    def countDifferentSubsequenceGCDs(self, nums: List[int]) -> int:
        isgcd = [False] * (max(nums) + 1)

        for i in nums:
            isgcd[i] = True

        for i in range(1, len(isgcd) // 2 + 1):
            mingcd = 0

            for j in range(i, len(isgcd), i):
                if isgcd[j]:
                    mingcd = gcd(mingcd, j) if mingcd > 0 else j
                if mingcd == i:
                    isgcd[i] = True
                    break

        return sum(isgcd)
