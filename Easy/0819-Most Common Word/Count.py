class Solution:
    def mostCommonWord(self, paragraph: str, banned: List[str]) -> str:
        banned = set(banned)
        counter = {}

        for word in re.sub("[!?',;.]", " ", paragraph).split():
            if word.lower() not in banned:
                if not counter.get(word.lower()):
                    counter[word.lower()] = 0
                counter[word.lower()] += 1

        return max(counter.items(), key=lambda item: item[1])[0]
