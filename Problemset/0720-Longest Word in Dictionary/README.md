# 720. Longest Word in Dictionary
Given a list of strings ```words``` representing an English Dictionary, find the longest word in ```words``` that can be built one character at a time by other words in ```words```. If there is more than one possible answer, return the longest word with the smallest lexicographical order.

If there is no answer, return the empty string.

#### Example 1:
<pre>
<strong>Input:</strong>
words = ["w","wo","wor","worl", "world"]
<strong>Output:</strong> "world"
<strong>Explanation:</strong>
The word "world" can be built one character at a time by "w", "wo", "wor", and "worl".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
words = ["a", "banana", "app", "appl", "ap", "apply", "apple"]
<strong>Output:</strong> "apple"
<strong>Explanation:</strong>
Both "apply" and "apple" can be built from other words in the dictionary. However, "apple" is lexicographically smaller than "apply".
</pre>

#### Note:
* All the strings in the input will only contain lowercase letters.
* The length of ```words``` will be in the range ```[1, 1000]```.
* The length of ```words[i]``` will be in the range ```[1, 30]```.

## Solutions (Python)

### 1. Brute Force
```Python3
class Solution:
    def longestWord(self, words: List[str]) -> str:
        words_set = set(words)
        ret = ""

        for word in words:
            temp = word
            while temp[:-1] in words_set:
                temp = temp[:-1]

            if not temp[:-1]:
                if len(word) > len(ret):
                    ret = word
                elif len(word) == len(ret) and word < ret:
                    ret = word

        return ret
```
