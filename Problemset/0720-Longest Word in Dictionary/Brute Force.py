class Solution:
    def longestWord(self, words: List[str]) -> str:
        words_set = set(words)
        ret = ""

        for word in words:
            temp = word
            while temp[:-1] in words_set:
                temp = temp[:-1]

            if not temp[:-1]:
                if len(word) > len(ret):
                    ret = word
                elif len(word) == len(ret) and word < ret:
                    ret = word

        return ret
