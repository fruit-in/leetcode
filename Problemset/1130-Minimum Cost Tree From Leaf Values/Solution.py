class Solution:
    def mctFromLeafValues(self, arr: List[int]) -> int:
        @cache
        def mctFromSub(i: int, j: int) -> (int, int):
            if i + 1 == j:
                return (0, arr[i])

            treesum, treemax = 1 << 31, 0

            for k in range(i + 1, j):
                leftsum, leftmax = mctFromSub(i, k)
                rightsum, rightmax = mctFromSub(k, j)
                treesum, treemax = min(
                    treesum, leftsum + rightsum + leftmax * rightmax), max(leftmax, rightmax)

            return (treesum, treemax)

        return mctFromSub(0, len(arr))[0]
