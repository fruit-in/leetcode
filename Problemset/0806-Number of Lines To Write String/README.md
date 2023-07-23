# 806. Number of Lines To Write String
We are to write the letters of a given string ```S```, from left to right into lines. Each line has maximum width 100 units, and if writing a letter would cause the width of the line to exceed 100 units, it is written on the next line. We are given an array ```widths```, an array where widths[0] is the width of 'a', widths[1] is the width of 'b', ..., and widths[25] is the width of 'z'.

Now answer two questions: how many lines have at least one character from ```S```, and what is the width used by the last such line? Return your answer as an integer list of length 2.

<pre>
<strong>Example:</strong>
<strong>Input:</strong>
widths = [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10]
S = "abcdefghijklmnopqrstuvwxyz"
<strong>Output:</strong> [3, 60]
<strong>Explanation:</strong>
All letters have the same length of 10. To write all 26 letters,
we need two full lines and one line with 60 units.
</pre>

<pre>
<strong>Example:</strong>
<strong>Input:</strong>
widths = [4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10]
S = "bbbcccdddaaa"
<strong>Output:</strong> [2, 4]
<strong>Explanation:</strong>
All letters except 'a' have the same length of 10, and
"bbbcccdddaa" will cover 9 * 10 + 2 * 4 = 98 units.
For the last 'a', it is written on the second line because
there is only 2 units left in the first line.
So the answer is 2 lines, plus 4 units in the second line.
</pre>

#### Note:
* The length of ```S``` will be in the range [1, 1000].
* ```S``` will only contain lowercase letters.
* ```widths``` is an array of length ```26```.
* ```widths[i]``` will be in the range of ```[2, 10]```.

## Solutions (Python)

### 1. Simulation
```Python3
class Solution:
    def numberOfLines(self, widths: List[int], S: str) -> List[int]:
        ret = [1, 0]
        for ch in S:
            if ret[1] + widths[ord(ch) - 97] > 100:
                ret[0] += 1
                ret[1] = 0
            ret[1] += widths[ord(ch) - 97]

        return ret
```
