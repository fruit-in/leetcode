import re


class Solution:
    def spellchecker(self, wordlist: List[str], queries: List[str]) -> List[str]:
        wordset = set(wordlist)
        capdict = {}
        vowdict = {}
        ret = []

        for word in wordlist:
            if word.lower() not in capdict:
                capdict[word.lower()] = word
            if re.sub(r"[aeiou]", "?", word.lower()) not in vowdict:
                vowdict[re.sub(r"[aeiou]", "?", word.lower())] = word

        for word in queries:
            if word in wordset:
                ret.append(word)
            elif word.lower() in capdict:
                ret.append(capdict[word.lower()])
            elif re.sub(r"[aeiou]", "?", word.lower()) in vowdict:
                ret.append(vowdict[re.sub(r"[aeiou]", "?", word.lower())])
            else:
                ret.append("")

        return ret
