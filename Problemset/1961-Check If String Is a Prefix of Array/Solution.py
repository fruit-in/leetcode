class Solution:
    def isPrefixString(self, s: str, words: List[str]) -> bool:
        if s == "":
            return True
        elif words == []:
            return False
        elif len(s) < len(words[0]):
            return False
        elif s[:len(words[0])] != words[0]:
            return False
        else:
            return self.isPrefixString(s[len(words[0]):], words[1:])
