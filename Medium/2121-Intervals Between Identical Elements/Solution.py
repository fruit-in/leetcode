class Solution:
    def getDistances(self, arr: List[int]) -> List[int]:
        indices_sums = {}
        indices = [0] * len(arr)
        intervals = [0] * len(arr)

        for i in range(len(arr)):
            if arr[i] in indices_sums:
                indices_sums[arr[i]].append(i + indices_sums[arr[i]][-1])
            else:
                indices_sums[arr[i]] = [i]
            indices[i] = len(indices_sums[arr[i]]) - 1

        for i in range(len(arr)):
            prefix_sum = indices_sums[arr[i]]
            intervals[i] = prefix_sum[-1] + 2 * i * indices[i] + 2 * i \
                - 2 * prefix_sum[indices[i]] - i * len(prefix_sum)

        return intervals
