class Solution:
    def printVertically(self, s: str) -> List[str]:
        words = s.split()
        max_len = max(len(word) for word in words)
        v_words = [''] * max_len

        for i in range(max_len):
            for word in words:
                v_words[i] += word[i] if len(word) > i else ' '

        return [word.rstrip() for word in v_words]
