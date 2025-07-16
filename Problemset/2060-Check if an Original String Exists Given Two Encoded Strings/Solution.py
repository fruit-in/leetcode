class Solution:
    def possiblyEquals(self, s1: str, s2: str) -> bool:
        @cache
        def isMatchWithSkips(i: int, j: int, diff: int) -> bool:
            if i == len(s1) and j == len(s2):
                return diff == 0
            elif j == len(s2) or diff < 0:
                if diff >= 0 or i == len(s1):
                    return False
                elif s1[i].islower():
                    return isMatchWithSkips(i + 1, j, diff + 1)
                else:
                    for k in range(3):
                        if i + k < len(s1) and s1[i:i + k + 1].isdigit() and isMatchWithSkips(i + k + 1, j, diff + int(s1[i:i + k + 1])):
                            return True
                    return False
            elif i == len(s1) or diff > 0:
                if diff <= 0 or j == len(s2):
                    return False
                elif s2[j].islower():
                    return isMatchWithSkips(i, j + 1, diff - 1)
                else:
                    for k in range(3):
                        if j + k < len(s2) and s2[j:j + k + 1].isdigit() and isMatchWithSkips(i, j + k + 1, diff - int(s2[j:j + k + 1])):
                            return True
                    return False
            else:
                if s1[i].islower() and s2[j].islower():
                    return s1[i] == s2[j] and isMatchWithSkips(i + 1, j + 1, diff)
                elif s1[i].isdigit():
                    for k in range(3):
                        if i + k < len(s1) and s1[i:i + k + 1].isdigit() and isMatchWithSkips(i + k + 1, j, diff + int(s1[i:i + k + 1])):
                            return True
                    return False
                else:
                    for k in range(3):
                        if j + k < len(s2) and s2[j:j + k + 1].isdigit() and isMatchWithSkips(i, j + k + 1, diff - int(s2[j:j + k + 1])):
                            return True
                    return False

        return isMatchWithSkips(0, 0, 0)
