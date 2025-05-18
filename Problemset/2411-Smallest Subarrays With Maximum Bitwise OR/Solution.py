class Solution:
    def smallestSubarrays(self, nums: List[int]) -> List[int]:
        def subarrayOR(j: int) -> int:
            maxor = 0
            for k in range(m):
                if prefixcount[j + 1][k] - prefixcount[i][k] > 0:
                    maxor |= 1 << k

            return maxor

        m = int(math.log2(max(1, max(nums)))) + 1
        n = len(nums)
        prefixcount = [[0] * m]
        answer = [0] * n

        for x in nums:
            prefixcount.append(prefixcount[-1].copy())

            for i in range(m):
                prefixcount[-1][i] += (x >> i) & 1

        for i in range(n):
            maxor = 0
            for j in range(m):
                if prefixcount[n][j] - prefixcount[i][j] > 0:
                    maxor |= 1 << j

            j = bisect.bisect_left(range(n), maxor, lo=i, key=subarrayOR)
            answer[i] = j - i + 1

        return answer
