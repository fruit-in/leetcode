class Solution:
    def reorderSpaces(self, text: str) -> str:
        words = list(filter(lambda x: x != '', text.split(' ')))
        if len(words) == 1:
            div, mod = 0, text.count(' ')
        else:
            div, mod = divmod(text.count(' '), len(words) - 1)

        return (div * ' ').join(words) + mod * ' '
