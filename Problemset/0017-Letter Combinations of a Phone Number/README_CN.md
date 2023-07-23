# 17. 电话号码的字母组合
给定一个仅包含数字 `2-9` 的字符串，返回所有它能表示的字母组合。

给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/original_images/17_telephone_keypad.png)

#### 示例:
<pre>
<strong>输入:</strong> "23"
<strong>输出:</strong> ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
</pre>

#### 说明:
尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。

## 题解 (Python)

### 1. 回溯
```Python
class Solution:
    digit_to_letters = {'2': 'abc', '3': 'def',
                        '4': 'ghi', '5': 'jkl',
                        '6': 'mno', '7': 'pqrs',
                        '8': 'tuv', '9': 'wxyz'}

    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        if len(digits) == 1:
            return list(self.digit_to_letters[digits])

        sub = self.letterCombinations(digits[1:])

        return [c + s for s in sub for c in self.digit_to_letters[digits[0]]]
```
