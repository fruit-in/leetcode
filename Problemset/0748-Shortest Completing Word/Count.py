class Solution:
    def shortestCompletingWord(self, licensePlate: str, words: List[str]) -> str:
        shortest = 16
        lp_arr = [0] * 26

        for ch in licensePlate:
            if ch.isalpha():
                lp_arr[ord(ch.lower()) - 97] += 1

        for word in words:
            tmp = list(lp_arr)

            for ch in word:
                tmp[ord(ch) - 97] -= 1

            if all(x <= 0 for x in tmp) and len(word) < shortest:
                shortest = len(word)
                first = word

        return first
