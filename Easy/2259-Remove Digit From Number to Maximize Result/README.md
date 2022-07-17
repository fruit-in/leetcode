# 2259. Remove Digit From Number to Maximize Result
You are given a string `number` representing a **positive integer** and a character `digit`.

Return *the resulting string after removing **exactly one occurrence** of* `digit` *from* `number` *such that the value of the resulting string in **decimal** form is **maximized***. The test cases are generated such that `digit` occurs at least once in `number`.

#### Example 1:
<pre>
<strong>Input:</strong> number = "123", digit = "3"
<strong>Output:</strong> "12"
<strong>Explanation:</strong> There is only one '3' in "123". After removing '3', the result is "12".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> number = "1231", digit = "1"
<strong>Output:</strong> "231"
<strong>Explanation:</strong> We can remove the first '1' to get "231" or remove the second '1' to get "123".
Since 231 > 123, we return "231".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> number = "551", digit = "5"
<strong>Output:</strong> "51"
<strong>Explanation:</strong> We can remove either the first or second '5' from "551".
Both result in the string "51".
</pre>

#### Constraints:
* `2 <= number.length <= 100`
* `number` consists of digits from `'1'` to `'9'`.
* `digit` is a digit from `'1'` to `'9'`.
* `digit` occurs at least once in `number`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def removeDigit(self, number: str, digit: str) -> str:
        i = 0

        for j in range(len(number)):
            if number[j] == digit:
                i = j
                if j + 1 < len(number) and number[j] < number[j + 1]:
                    break

        return number[:i] + number[i + 1:]
```
