class Solution:
    def findWords(self, words: List[str]) -> List[str]:
        result = []
        row_q = set("qwertyuiop")
        row_a = set("asdfghjkl")
        row_z = set("zxcvbnm")
        for word in words:
            word_set = set(word.lower())
            if word_set <= row_q or word_set <= row_a or word_set <= row_z:
                result.append(word)
        return result
