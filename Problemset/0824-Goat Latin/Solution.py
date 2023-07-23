class Solution:
    def toGoatLatin(self, S: str) -> str:
        words = S.split(' ')
        for i, word in enumerate(words):
            if word[0].lower() not in 'aeiou':
                words[i] = word[1:] + word[:1]
            words[i] += 'maa' + 'a' * i
        return ' '.join(words)
