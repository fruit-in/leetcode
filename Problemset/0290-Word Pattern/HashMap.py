class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        words = str.split()

        if len(pattern) != len(words):
            return False

        match = {}
        used = set()

        for ch, wo in zip(pattern, words):
            if (ch in match) != (wo in used):
                return False
            elif ch not in match:
                match[ch] = wo
                used.add(wo)
            elif match[ch] != wo:
                return False

        return True
