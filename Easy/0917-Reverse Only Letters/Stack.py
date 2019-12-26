class Solution:
    def reverseOnlyLetters(self, S: str) -> str:
        stack = []
        ret = ""

        for ch in S:
            if ch.isalpha():
                stack.append(ch)

        for ch in S:
            if ch.isalpha():
                ret += stack.pop()
            else:
                ret += ch

        return ret
