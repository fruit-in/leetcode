class Solution:
    def putMarbles(self, weights: List[int], k: int) -> int:
        weightsij = sorted(weights[i] + weights[i + 1]
                           for i in range(len(weights) - 1))

        return sum(weightsij[len(weights) - k:]) - sum(weightsij[:k - 1])
