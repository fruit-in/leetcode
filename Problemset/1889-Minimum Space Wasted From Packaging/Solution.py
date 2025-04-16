class Solution:
    def minWastedSpace(self, packages: List[int], boxes: List[List[int]]) -> int:
        prefixsum = [0] * (len(packages) + 1)
        ret = float("inf")

        packages.sort()

        for i in range(len(packages)):
            prefixsum[i + 1] = prefixsum[i] + packages[i]

        for supplier in boxes:
            wasted = 0
            i = 0
            supplier.sort()

            if supplier[-1] < packages[-1]:
                continue

            for size in supplier:
                j = bisect.bisect(packages, size, lo=i)
                wasted += (j - i) * size - prefixsum[j] + prefixsum[i]
                i = j

            ret = min(ret, wasted)

        return -1 if ret == float("inf") else ret % 1000000007
