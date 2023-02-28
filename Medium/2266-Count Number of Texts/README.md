# 2266. Count Number of Texts
Alice is texting Bob using her phone. The **mapping** of digits to letters is shown in the figure below.

![](https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png)

In order to **add** a letter, Alice has to **press** the key of the corresponding digit `i` times, where `i` is the position of the letter in the key.

* For example, to add the letter `'s'`, Alice has to press `'7'` four times. Similarly, to add the letter `'k'`, Alice has to press `'5'` twice.
* Note that the digits `'0'` and `'1'` do not map to any letters, so Alice **does not** use them.

However, due to an error in transmission, Bob did not receive Alice's text message but received a **string of pressed keys** instead.

* For example, when Alice sent the message `"bob"`, Bob received the string `"2266622"`.

Given a string pressedKeys representing the string received by Bob, return *the **total number of possible text messages** Alice could have sent*.

Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> pressedKeys = "22233"
<strong>Output:</strong> 8
<strong>Explanation:</strong>
The possible text messages Alice could have sent are:
"aaadd", "abdd", "badd", "cdd", "aaae", "abe", "bae", and "ce".
Since there are 8 possible messages, we return 8.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> pressedKeys = "222222222222222222222222222222222222"
<strong>Output:</strong> 82876089
<strong>Explanation:</strong>
There are 2082876103 possible text messages Alice could have sent.
Since we need to return the answer modulo 10<sup>9</sup> + 7, we return 2082876103 % (10<sup>9</sup> + 7) = 82876089.
</pre>

#### Constraints:
* <code>1 <= pressedKeys.length <= 10<sup>5</sup></code>
* `pressedKeys` only consists of digits from `'2'` - `'9'`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countTexts(self, pressedKeys: str) -> int:
        digit = '0'
        count = 0
        max3 = [1, 1, 2]
        max4 = [1, 1, 2, 4]
        ret = 1

        for d in (pressedKeys + '0'):
            if d == digit:
                count += 1
            else:
                if digit in "79":
                    while count >= len(max4):
                        max4.append(sum(max4[-4:]))
                    ret *= max4[count]
                else:
                    while count >= len(max3):
                        max3.append(sum(max3[-3:]))
                    ret *= max3[count]

                digit = d
                count = 1

        return ret % 1_000_000_007
```
