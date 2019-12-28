# 953. Verifying an Alien Dictionary
In an alien language, surprisingly they also use english lowercase letters, but possibly in a different ```order```. The ```order``` of the alphabet is some permutation of lowercase letters.

Given a sequence of ```words``` written in the alien language, and the ```order``` of the alphabet, return ```true``` if and only if the given ```words``` are sorted lexicographicaly in this alien language.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
<strong>Output:</strong> true
<strong>Explanation:</strong> As 'h' comes before 'l' in this language, then the sequence is sorted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
<strong>Output:</strong> false
<strong>Explanation:</strong> As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
<strong>Output:</strong> false
<strong>Explanation:</strong> The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '∅', where '∅' is defined as the blank character which is less than any other character (<a href="https://en.wikipedia.org/wiki/Lexicographical_order">More info</a>).
</pre>

#### Constraints:
* ```1 <= words.length <= 100```
* ```1 <= words[i].length <= 20```
* ```order.length == 26```
* All characters in ```words[i]``` and ```order``` are English lowercase letters.

## Solutions (Python)

### 1. Check Adjacent Words
```Python
class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        alien_order = [0] * 26
        for i in range(len(order)):
            alien_order[ord(order[i]) - 97] = i

        for i in range(len(words) - 1):
            for j in range(len(words[i])):
                if (j == len(words[i + 1]) or
                    alien_order[ord(words[i][j]) - 97] >
                    alien_order[ord(words[i + 1][j]) - 97]):
                    return False
                elif (alien_order[ord(words[i][j]) - 97] <
                      alien_order[ord(words[i + 1][j]) - 97]):
                    break

        return True
```

### 2. Translate
```Python
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
```
