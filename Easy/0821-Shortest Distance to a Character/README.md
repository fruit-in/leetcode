# 821. Shortest Distance to a Character
Given a string ```S``` and a character ```C```, return an array of integers representing the shortest distance from the character ```C``` in the string.

#### Example 1:
<pre>
<strong>Input:</strong> S = "loveleetcode", C = 'e'
<strong>Output:</strong> [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
</pre>

#### Note:
1. ```S``` string length is in ```[1, 10000]```.
2. ```C``` is a single character, and guaranteed to be in string ```S```.
3. All letters in ```S``` and ```C``` are lowercase.

## Solutions (Python)

### 1. Brute Force
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        ret = []

        for i in range(len(S)):
            shortest = len(S)

            for j in range(len(S)):
                if S[j] == C:
                    shortest = min(shortest, abs(i - j))

            ret.append(shortest)

        return ret
```

### 2. Find both Sides
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        ret = []

        for i in range(len(S)):
            for j in range(len(S)):
                l_char = S[i - j] if i - j >= 0 else ''
                r_char = S[i + j] if i + j < len(S) else ''

                if l_char == C or r_char == C:
                    ret.append(j)
                    break

        return ret
```

### 3. Min Array
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        prev = len(S)
        left = []

        for ch in S:
            prev += 1
            if ch == C:
                prev = 0
            left.append(prev)

        prev = len(S)
        right = []

        for ch in S[::-1]:
            prev += 1
            if ch == C:
                prev = 0
            right.append(prev)
        right.reverse()

        return [min(left[i], right[i]) for i in range(len(S))]
```

### 4. Three Pointers
```Python3
class Solution:
    def shortestToChar(self, S: str, C: str) -> List[int]:
        prev = -len(S)
        next = S.find(C)
        ret = []

        for i in range(len(S)):
            ret.append(min(i - prev, next - i))

            if i == next:
                prev = next
                try:
                    next = S.index(C, next + 1)
                except:
                    next = 2 * len(S)

        return ret
```
