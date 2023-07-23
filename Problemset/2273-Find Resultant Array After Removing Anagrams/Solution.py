class Solution:
    def removeAnagrams(self, words: List[str]) -> List[str]:
        ret = [words[0]]

        for word in words:
            if Counter(ret[-1]) != Counter(word):
                ret.append(word)

        return ret
