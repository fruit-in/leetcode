# 966. Vowel Spellchecker
Given a `wordlist`, we want to implement a spellchecker that converts a query word into a correct word.

For a given `query` word, the spell checker handles two categories of spelling mistakes:

* Capitalization: If the query matches a word in the wordlist (**case-insensitive**), then the query word is returned with the same case as the case in the wordlist.
    * Example: `wordlist = ["yellow"]`, `query = "YellOw"`: `correct = "yellow"`
    * Example: `wordlist = ["Yellow"]`, `query = "yellow"`: `correct = "Yellow"`
    * Example: `wordlist = ["yellow"]`, `query = "yellow"`: `correct = "yellow"`
* Vowel Errors: If after replacing the vowels `('a', 'e', 'i', 'o', 'u')` of the query word with any vowel individually, it matches a word in the wordlist (**case-insensitive**), then the query word is returned with the same case as the match in the wordlist.
    * Example: `wordlist = ["YellOw"]`, `query = "yollow"`: `correct = "YellOw"`
    * Example: `wordlist = ["YellOw"]`, `query = "yeellow"`: `correct = ""` (no match)
    * Example: `wordlist = ["YellOw"]`, `query = "yllw"`: `correct = ""` (no match)

In addition, the spell checker operates under the following precedence rules:

* When the query exactly matches a word in the wordlist (**case-sensitive**), you should return the same word back.
* When the query matches a word up to capitlization, you should return the first such match in the wordlist.
* When the query matches a word up to vowel errors, you should return the first such match in the wordlist.
* If the query has no matches in the wordlist, you should return the empty string.

Given some `queries`, return a list of words `answer`, where `answer[i]` is the correct word for `query = queries[i]`.

#### Example 1:
<pre>
<strong>Input:</strong> wordlist = ["KiTe","kite","hare","Hare"], queries = ["kite","Kite","KiTe","Hare","HARE","Hear","hear","keti","keet","keto"]
<strong>Output:</strong> ["kite","KiTe","KiTe","Hare","hare","","","KiTe","","KiTe"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> wordlist = ["yellow"], queries = ["YellOw"]
<strong>Output:</strong> ["yellow"]
</pre>

#### Constraints:
* `1 <= wordlist.length, queries.length <= 5000`
* `1 <= wordlist[i].length, queries[i].length <= 7`
* `wordlist[i]` and `queries[i]` consist only of only English letters.

## Solutions (Python)

### 1. Solution
```Python
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
```
