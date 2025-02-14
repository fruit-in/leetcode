class Solution:
    def numFactoredBinaryTrees(self, arr: List[int]) -> int:
        count = {}
        ret = 0

        arr.sort()

        for i in range(len(arr)):
            count[arr[i]] = 1

            for j in range(i):
                if arr[i] % arr[j] == 0:
                    count[arr[i]] = (
                        count[arr[i]] + count[arr[j]] * count.get(arr[i] // arr[j], 0)) % 1_000_000_007

            ret = (ret + count[arr[i]]) % 1_000_000_007

        return ret
