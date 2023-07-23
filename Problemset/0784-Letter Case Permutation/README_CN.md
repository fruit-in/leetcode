# 784. 字母大小写全排列
给定一个字符串```S```，通过将字符串```S```中的每个字母转变大小写，我们可以获得一个新的字符串。返回所有可能得到的字符串集合。

<pre>
<strong>示例:</strong>
<strong>输入:</strong> S = "a1b2"
<strong>输出:</strong> ["a1b2", "a1B2", "A1b2", "A1B2"]

<strong>输入:</strong> S = "3z4"
<strong>输出:</strong> ["3z4", "3Z4"]

<strong>输入:</strong> S = "12345"
<strong>输出:</strong> ["12345"]
</pre>

#### 注意:
* ```S``` 的长度不超过```12```。
* ```S``` 仅由数字和字母组成。

## 题解 (Python)

### 1. 题解
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
