class Solution:
    def isValid(self, s: str) -> bool:
        while s.count("()") or s.count("[]") or s.count("{}"):
            s = s.replace("()", "")
            s = s.replace("[]", "")
            s = s.replace("{}", "")
        return not s
