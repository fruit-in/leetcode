class Solution:
    def numOfStrings(self, patterns: List[str], word: str) -> int:
        return len([p for p in patterns if p in word])
