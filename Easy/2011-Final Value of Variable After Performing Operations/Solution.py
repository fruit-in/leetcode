class Solution:
    def finalValueAfterOperations(self, operations: List[str]) -> int:
        return sum(44 - ord(o[1]) for o in operations)
