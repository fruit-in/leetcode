# 17. Letter Combinations of a Phone Number
Given a string containing digits from `2-9` inclusive, return all possible letter combinations that the number could represent.

A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

![](https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png)

#### Example:
<pre>
<strong>Input:</strong> "23"
<strong>Output:</strong> ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
</pre>

#### Note:
Although the above answer is in lexicographical order, your answer could be in any order you want.

## Solutions (Python)

### 1. Backtracking
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
