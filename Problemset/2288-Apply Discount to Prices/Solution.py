class Solution:
    def discountPrices(self, sentence: str, discount: int) -> str:
        words = sentence.split()
        discount = 1 - discount / 100

        for i in range(len(words)):
            if words[i][0] == '$':
                try:
                    words[i] = "${:.2f}".format(int(words[i][1:]) * discount)
                except:
                    pass

        return ' '.join(words)
