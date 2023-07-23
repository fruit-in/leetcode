class Solution:
    def isValid(self, S: str) -> bool:
        while 'abc' in S:
            S = S.replace('abc', '')

        return not S
