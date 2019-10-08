# 1078. Occurrences After Bigram
Given words ```first``` and ```second```, consider occurrences in some ```text``` of the form "```first second third```", where ```second``` comes immediately after ```first```, and ```third``` comes immediately after ```second```.

For each such occurrence, add "```third```" to the answer, and return the answer.

#### Example 1:
<pre>
<strong>Input:</strong> text = "alice is a good girl she is a good student", first = "a", second = "good"
<strong>Output:</strong> ["girl","student"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> text = "we will we will rock you", first = "we", second = "will"
<strong>Output:</strong> ["we","rock"]
</pre>

#### Note:
1. ```1 <= text.length <= 1000```
2. ```text``` consists of space separated words, where each word consists of lowercase English letters.
3. ```1 <= first.length, second.length <= 10```
4. ```first``` and ```second``` consist of lowercase English letters.

## Solutions (Python)

### 1. Linear Scan
```Python3
class Solution:
    def findOcurrences(self, text: str, first: str, second: str) -> List[str]:
        third = []
        words = text.split()

        for i in range(len(words) - 2):
            if words[i] == first and words[i + 1] == second:
                third.append(words[i + 2])

        return third
```
