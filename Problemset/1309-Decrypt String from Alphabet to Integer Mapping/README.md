# 1309. Decrypt String from Alphabet to Integer Mapping
Given a string ```s``` formed by digits (```'0'``` - ```'9'```) and ```'#'``` . We want to map ```s``` to English lowercase characters as follows:
* Characters (```'a'``` to ```'i'```) are represented by (```'1'``` to ```'9'```) respectively.
* Characters (```'j'``` to ```'z'```) are represented by (```'10#'``` to ```'26#'```) respectively.

Return the string formed after mapping.

It's guaranteed that a unique mapping will always exist.

#### Example 1:
<pre>
<strong>Input:</strong> s = "10#11#12"
<strong>Output:</strong> "jkab"
<strong>Explanation:</strong> "j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "1326#"
<strong>Output:</strong> "acz"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "25#"
<strong>Output:</strong> "y"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"
<strong>Output:</strong> "abcdefghijklmnopqrstuvwxyz"
</pre>

#### Constraints:
* ```1 <= s.length <= 1000```
* ```s[i]``` only contains digits letters (```'0'```-```'9'```) and ```'#'``` letter.
* ```s``` will be valid string such that mapping is always possible.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def freqAlphabets(self, s: str) -> str:
        ret = ""

        i = 0
        while i < len(s):
            if i + 2 < len(s) and s[i + 2] == '#':
                ret += chr(int(s[i:i + 2]) + 96)
                i += 3
            else:
                ret += chr(int(s[i]) + 96)
                i += 1

        return ret
```
