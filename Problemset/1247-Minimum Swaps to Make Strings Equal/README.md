# 1247. Minimum Swaps to Make Strings Equal
You are given two strings ```s1``` and ```s2``` of equal length consisting of letters ```"x"``` and ```"y"``` **only**. Your task is to make these two strings equal to each other. You can swap any two characters that belong to **different** strings, which means: swap ```s1[i]``` and ```s2[j]```.

Return the minimum number of swaps required to make ```s1``` and ```s2``` equal, or return ```-1``` if it is impossible to do so.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "xx", s2 = "yy"
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Swap s1[0] and s2[1], s1 = "yx", s2 = "yx".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "xy", s2 = "yx"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
Swap s1[0] and s2[0], s1 = "yy", s2 = "xx".
Swap s1[0] and s2[1], s1 = "xy", s2 = "xy".
Note that you can't swap s1[0] and s1[1] to make s1 equal to "yx", cause we can only swap chars in different strings.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s1 = "xx", s2 = "xy"
<strong>Output:</strong> -1
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s1 = "xxyyxyxyxx", s2 = "xyyxyxxxyx"
<strong>Output:</strong> 4
</pre>

#### Constraints:
* ```1 <= s1.length, s2.length <= 1000```
* ```s1, s2``` only contain ```'x'``` or ```'y'```.

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def minimumSwap(self, s1: str, s2: str) -> int:
        xy, yx = 0, 0

        for i in range(len(s1)):
            if s1[i] == 'x' and s2[i] == 'y':
                xy += 1
            elif s1[i] == 'y' and s2[i] == 'x':
                yx += 1

        ret = xy // 2 + yx // 2

        if xy % 2 == yx % 2:
            return ret + xy % 2 * 2
        else:
            return -1
```
