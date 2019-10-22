# 1002. Find Common Characters
Given an array ```A``` of strings made only from lowercase letters, return a list of all characters that show up in all strings within the list **(including duplicates)**.  For example, if a character occurs 3 times in all strings but not 4 times, you need to include that character three times in the final answer.

You may return the answer in any order.

#### Example 1:
<pre>
<strong>Input:</strong> ["bella","label","roller"]
<strong>Output:</strong> ["e","l","l"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ["cool","lock","cook"]
<strong>Output:</strong> ["c","o"]
</pre>

#### Note:
1. ```1 <= A.length <= 100```
2. ```1 <= A[i].length <= 100```
3. ```A[i][j]``` is a lowercase letter

## Solutions (Python)

### 1. Count
```Python
class Solution:
    def commonChars(self, A: List[str]) -> List[str]:
        cnt1 = [100] * 26

        for wo in A:
            cnt2 = [0] * 26
            for ch in wo:
                cnt2[ord(ch) - 97] += 1

            for i in range(26):
                cnt1[i] = min(cnt1[i], cnt2[i])

        ret = []

        for i in range(26):
            ret.extend([chr(97 + i)] * cnt1[i])

        return ret
```
