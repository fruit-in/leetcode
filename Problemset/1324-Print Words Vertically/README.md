# 1324. Print Words Vertically
Given a string ```s```. Return all the words vertically in the same order in which they appear in ```s```.
Words are returned as a list of strings, complete with spaces when is necessary. (Trailing spaces are not allowed).
Each word would be put on only one column and that in one column there will be only one word.

#### Example 1:
<pre>
<strong>Input:</strong> s = "HOW ARE YOU"
<strong>Output:</strong> ["HAY","ORO","WEU"]
<strong>Explanation:</strong> Each word is printed vertically. 
 "HAY"
 "ORO"
 "WEU"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "TO BE OR NOT TO BE"
<strong>Output:</strong> ["TBONTB","OEROOE","   T"]
<strong>Explanation:</strong> Trailing spaces is not allowed. 
"TBONTB"
"OEROOE"
"   T"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "CONTEST IS COMING"
<strong>Output:</strong> ["CIC","OSO","N M","T I","E N","S G","T"]
</pre>

#### Constraints:
* ```1 <= s.length <= 200```
* ```s``` contains only upper case English letters.
* It's guaranteed that there is only one space between 2 words.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def printVertically(self, s: str) -> List[str]:
        words = s.split()
        max_len = max(len(word) for word in words)
        v_words = [''] * max_len

        for i in range(max_len):
            for word in words:
                v_words[i] += word[i] if len(word) > i else ' '

        return [word.rstrip() for word in v_words]
```
