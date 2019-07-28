class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        if str1 == str2:
            return str1
        if str1.startswith(str2):
            return self.gcdOfStrings(str1[len(str2):], str2)
        elif str2.startswith(str1):
            return self.gcdOfStrings(str1, str2[len(str1):])
        return ""
