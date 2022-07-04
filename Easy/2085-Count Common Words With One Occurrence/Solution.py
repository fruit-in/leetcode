class Solution:
    def countWords(self, words1: List[str], words2: List[str]) -> int:
        count1 = Counter(words1)
        count2 = Counter(words2)

        return sum(1 for k, v in count1.items() if v == 1 and count2[k] == 1)
