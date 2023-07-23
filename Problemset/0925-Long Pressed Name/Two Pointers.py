class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        i = 0
        for c in typed:
            if i < len(name) and name[i] == c:
                i += 1
            elif i == 0 or name[i - 1] != c:
                return False

        return i == len(name)
