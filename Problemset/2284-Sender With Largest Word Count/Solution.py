class Solution:
    def largestWordCount(self, messages: List[str], senders: List[str]) -> str:
        wordcount = {}

        for (sender, message) in zip(senders, messages):
            if sender not in wordcount:
                wordcount[sender] = 0
            wordcount[sender] += len(message.split())

        return max(senders, key=lambda s: (wordcount[s], s))
