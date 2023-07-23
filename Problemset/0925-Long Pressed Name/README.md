# 925. Long Pressed Name
Your friend is typing his ```name``` into a keyboard.  Sometimes, when typing a character ```c```, the key might get *long pressed*, and the character will be typed 1 or more times.

You examine the ```typed``` characters of the keyboard.  Return ```True``` if it is possible that it was your friends name, with some characters (possibly none) being long pressed.

#### Example 1:
<pre>
<strong>Input:</strong> name = "alex", typed = "aaleex"
<strong>Output:</strong> true
<strong>Explanation:</strong> 'a' and 'e' in 'alex' were long pressed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> name = "saeed", typed = "ssaaedd"
<strong>Output:</strong> false
<strong>Explanation:</strong> 'e' must have been pressed twice, but it wasn't in the typed output.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> name = "leelee", typed = "lleeelee"
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> name = "laiden", typed = "laiden"
<strong>Output:</strong> true
<strong>Explanation:</strong> It's not necessary to long press any character.
</pre>

#### Note:
1. ```name.length <= 1000```
2. ```typed.length <= 1000```
3. The characters of ```name``` and ```typed``` are lowercase letters.

## Solutions (Python)

### 1. Group into Blocks
```Python
class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        n_cnt = []
        cnt = 1
        for i in range(len(name)):
            if i < len(name) - 1 and name[i] == name[i + 1]:
                cnt += 1
            else:
                n_cnt.append((name[i], cnt))
                cnt = 1

        t_cnt = []
        cnt = 1
        for i in range(len(typed)):
            if i < len(typed) - 1 and typed[i] == typed[i + 1]:
                cnt += 1
            else:
                t_cnt.append((typed[i], cnt))
                cnt = 1

        return all(n[0] == t[0] and n[1] <= t[1] for n, t in zip(n_cnt, t_cnt)) and len(n_cnt) == len(t_cnt)
```

### 2. Two Pointers
```Python
class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        i = 0
        for c in typed:
            if i < len(name) and name[i] == c:
                i += 1
            elif i == 0 or name[i - 1] != c:
                return False

        return i == len(name)
```
