# 2042. Check if Numbers Are Ascending in a Sentence
A sentence is a list of **tokens** separated by a **single** space with no leading or trailing spaces. Every token is either a **positive number** consisting of digits `0-9` with no leading zeros, or a **word** consisting of lowercase English letters.
* For example, `"a puppy has 2 eyes 4 legs"` is a sentence with seven tokens: `"2"` and `"4"` are numbers and the other tokens such as `"puppy"` are words.

Given a string `s` representing a sentence, you need to check if **all** the numbers in `s` are **strictly increasing** from left to right (i.e., other than the last number, **each** number is **strictly smaller** than the number on its **right** in `s`).

Return `true` *if so, or* `false` *otherwise*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/09/30/example1.png)
<pre>
<strong>Input:</strong> s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles"
<strong>Output:</strong> true
<strong>Explanation:</strong> The numbers in s are: 1, 3, 4, 6, 12.
They are strictly increasing from left to right: 1 < 3 < 4 < 6 < 12.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "hello world 5 x 5"
<strong>Output:</strong> false
<strong>Explanation:</strong> The numbers in s are: 5, 5. They are not strictly increasing.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/09/30/example3.png)
<pre>
<strong>Input:</strong> s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
<strong>Output:</strong> false
<strong>Explanation:</strong> The numbers in s are: 7, 51, 50, 60. They are not strictly increasing.
</pre>

#### Constraints:
* `3 <= s.length <= 200`
* `s` consists of lowercase English letters, spaces, and digits from `0` to `9`, inclusive.
* The number of tokens in `s` is between `2` and `100`, inclusive.
* The tokens in `s` are separated by a single space.
* There are at least **two** numbers in `s`.
* Each number in `s` is a **positive** number **less** than `100`, with no leading zeros.
* `s` contains no leading or trailing spaces.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def areNumbersAscending(self, s: str) -> bool:
        nums = [int(token) for token in s.split() if token.isdecimal()]

        return all(nums[i - 1] < nums[i] for i in range(1, len(nums)))
```
