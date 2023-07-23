class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        prev = len(S)
        left = []

        for ch in S:
            prev += 1
            if ch == C:
                prev = 0
            left.append(prev)

        prev = len(S)
        right = []

        for ch in S[::-1]:
            prev += 1
            if ch == C:
                prev = 0
            right.append(prev)
        right.reverse()

        return [min(left[i], right[i]) for i in range(len(S))]
