class Solution:
    def backspaceCompare(self, S: str, T: str) -> bool:
        stack_s = []
        stack_t = []

        for ch in S:
            if ch != '#':
                stack_s.append(ch)
            elif stack_s:
                stack_s.pop()
        for ch in T:
            if ch != '#':
                stack_t.append(ch)
            elif stack_t:
                stack_t.pop()

        return stack_s == stack_t
