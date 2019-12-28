class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        alphabet = "abcdefghijklmnopqrtsuvwxyz"

        prev = ""
        for curr in words:
            word = curr.translate(curr.maketrans(order, alphabet))

            if prev > word:
                return False

            prev = word

        return True
