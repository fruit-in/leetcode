# 1002. 查找常用字符
给定仅有小写字母组成的字符串数组 ```A```，返回列表中的每个字符串中都显示的全部字符（**包括重复字符**）组成的列表。例如，如果一个字符在每个字符串中出现 3 次，但不是 4 次，则需要在最终答案中包含该字符 3 次。

你可以按任意顺序返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> ["bella","label","roller"]
<strong>输出:</strong> ["e","l","l"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ["cool","lock","cook"]
<strong>输出:</strong> ["c","o"]
</pre>

#### 提示:
1. ```1 <= A.length <= 100```
2. ```1 <= A[i].length <= 100```
3. ```A[i][j]``` 是小写字母

## 题解 (Python)

### 1. 计数
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
