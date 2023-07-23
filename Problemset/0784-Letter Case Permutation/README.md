# 784. Letter Case Permutation
Given a string S, we can transform every letter individually to be lowercase or uppercase to create another string.  Return a list of all possible strings we could create.

<pre>
<strong>Examples:</strong>
<strong>Input:</strong> S = "a1b2"
<strong>Output:</strong> ["a1b2", "a1B2", "A1b2", "A1B2"]

<strong>Input:</strong> S = "3z4"
<strong>Output:</strong> ["3z4", "3Z4"]

<strong>Input:</strong> S = "12345"
<strong>Output:</strong> ["12345"]
</pre>

#### Note:
* ```S``` will be a string with length between ```1``` and ```12```.
* ```S``` will consist only of letters or digits.

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def letterCasePermutation(self, S: str) -> List[str]:
        ret = [""]

        for ch in S:
            if ch.isalpha():
                tmp = [s + ch.lower() for s in ret]
                ret = [s + ch.upper() for s in ret]
                ret.extend(tmp)
            else:
                ret = [s + ch for s in ret]

        return ret
```
