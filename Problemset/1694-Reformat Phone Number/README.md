# 1694. Reformat Phone Number
You are given a phone number as a string `number`. `number` consists of digits, spaces `' '`, and/or dashes `'-'`.

You would like to reformat the phone number in a certain manner. Firstly, **remove** all spaces and dashes. Then, **group** the digits from left to right into blocks of length 3 **until** there are 4 or fewer digits. The final digits are then grouped as follows:
* 2 digits: A single block of length 2.
* 3 digits: A single block of length 3.
* 4 digits: Two blocks of length 2 each.

The blocks are then joined by dashes. Notice that the reformatting process should **never** produce any blocks of length 1 and produce **at most** two blocks of length 2.

Return *the phone number after formatting*.

#### Example 1:
<pre>
<strong>Input:</strong> number = "1-23-45 6"
<strong>Output:</strong> "123-456"
<strong>Explanation:</strong> The digits are "123456".
Step 1: There are more than 4 digits, so group the next 3 digits. The 1st block is "123".
Step 2: There are 3 digits remaining, so put them in a single block of length 3. The 2nd block is "456".
Joining the blocks gives "123-456".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> number = "123 4-567"
<strong>Output:</strong> "123-45-67"
<strong>Explanation:</strong> The digits are "1234567".
Step 1: There are more than 4 digits, so group the next 3 digits. The 1st block is "123".
Step 2: There are 4 digits left, so split them into two blocks of length 2. The blocks are "45" and "67".
Joining the blocks gives "123-45-67".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> number = "123 4-5678"
<strong>Output:</strong> "123-456-78"
<strong>Explanation:</strong> The digits are "12345678".
Step 1: The 1st block is "123".
Step 2: The 2nd block is "456".
Step 3: There are 2 digits left, so put them in a single block of length 2. The 3rd block is "78".
Joining the blocks gives "123-456-78".
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> number = "12"
<strong>Output:</strong> "12"
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> number = "--17-5 229 35-39475 "
<strong>Output:</strong> "175-229-353-94-75"
</pre>

#### Constraints:
* `2 <= number.length <= 100`
* `number` consists of digits and the characters `'-'` and `' '`.
* There are at least **two** digits in `number`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def reformatNumber(self, number: str) -> str:
        digits = number.replace("-", "").replace(" ", "")
        blocks = [digits[i:i + 3] for i in range(0, len(digits), 3)]

        if len(blocks[-1]) == 1:
            blocks[-1] = blocks[-2][2] + blocks[-1]
            blocks[-2] = blocks[-2][:2]

        return "-".join(blocks)
```
