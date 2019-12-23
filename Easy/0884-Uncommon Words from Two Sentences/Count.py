class Solution:
    def uncommonFromSentences(self, A: str, B: str) -> List[str]:
        word_count = {}
        ret = []

        for word in (A + ' ' + B).split(' '):
            if word_count.get(word):
                word_count[word] += 1
            else:
                word_count[word] = 1

        for word, count in word_count.items():
            if count == 1:
                ret.append(word)

        return ret
