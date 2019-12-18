# 819. Most Common Word
Given a paragraph and a list of banned words, return the most frequent word that is not in the list of banned words.  It is guaranteed there is at least one word that isn't banned, and that the answer is unique.

Words in the list of banned words are given in lowercase, and free of punctuation.  Words in the paragraph are not case sensitive.  The answer is in lowercase.

#### Example:
<pre>
<strong>Input:</strong>
paragraph = "Bob hit a ball, the hit BALL flew far after it was hit."
banned = ["hit"]
<strong>Output:</strong> "ball"
<strong>Explanation:</strong>
"hit" occurs 3 times, but it is a banned word.
"ball" occurs twice (and no other word does), so it is the most frequent non-banned word in the paragraph.
Note that words in the paragraph are not case sensitive,
that punctuation is ignored (even if adjacent to words, such as "ball,"),
and that "hit" isn't the answer even though it occurs more because it is banned.
</pre>

#### Note:
* ```1 <= paragraph.length <= 1000```.
* ```0 <= banned.length <= 100```.
* ```1 <= banned[i].length <= 10```.
* The answer is unique, and written in lowercase (even if its occurrences in ```paragraph``` may have uppercase symbols, and even if it is a proper noun.)
* ```paragraph``` only consists of letters, spaces, or the punctuation symbols ```!?',;.```
* There are no hyphens or hyphenated words.
* Words only consist of letters, never apostrophes or other punctuation symbols.

## Solutions (Python)

### 1. Count
```Python
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
```
