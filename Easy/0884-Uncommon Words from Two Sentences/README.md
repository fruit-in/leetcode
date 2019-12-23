# 884. Uncommon Words from Two Sentences
We are given two sentences ```A``` and ```B```.  (A *sentence* is a string of space separated words.  Each *word* consists only of lowercase letters.)

A word is *uncommon* if it appears exactly once in one of the sentences, and does not appear in the other sentence.

Return a list of all uncommon words.

You may return the list in any order.

#### Example 1:
<pre>
<strong>Input:</strong> A = "this apple is sweet", B = "this apple is sour"
<strong>Output:</strong> ["sweet","sour"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = "apple apple", B = "banana"
<strong>Output:</strong> ["banana"]
</pre>

#### Note:
1. ```0 <= A.length <= 200```
2. ```0 <= B.length <= 200```
3. ```A``` and ```B``` both contain only spaces and lowercase letters.

## Solutions (Python)

### 1. Count
```Python
class Solution:
    def uncommonFromSentences(self, A: str, B: str) -> List[str]:
        word_count = {}
        ret = []

        for word in (A + ' ' + B).split(' '):
            if word_count.get(word):
                word_count[word] += 1
            else:
                word_count[word] = 1

        for word, count in word_count.items():
            if count == 1:
                ret.append(word)

        return ret
```
