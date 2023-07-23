class Solution:
    def minDeletionSize(self, A: List[str]) -> int:
        return len(list(filter(lambda B : B != sorted(B), map(list, zip(*A)))))
