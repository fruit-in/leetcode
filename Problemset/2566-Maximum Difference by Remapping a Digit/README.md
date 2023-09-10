# 2566. Maximum Difference by Remapping a Digit
You are given an integer `num`. You know that Danny Mittal will sneakily **remap** one of the `10` possible digits (`0` to `9`) to another digit.

Return *the difference between the maximum and minimum values Danny can make by remapping **exactly one** digit in* `num`.

**Notes:**

* When Danny remaps a digit `d1` to another digit `d2`, Danny replaces all occurrences of `d1` in num with `d2`.
* Danny can remap a digit to itself, in which case `num` does not change.
* Danny can remap different digits for obtaining minimum and maximum values respectively.
* The resulting number after remapping can contain leading zeroes.
* We mentioned "Danny Mittal" to congratulate him on being in the top 10 in Weekly Contest 326.

#### Example 1:
<pre>
<strong>Input:</strong> num = 11891
<strong>Output:</strong> 99009
<strong>Explanation:</strong>
To achieve the maximum value, Danny can remap the digit 1 to the digit 9 to yield 99899.
To achieve the minimum value, Danny can remap the digit 1 to the digit 0, yielding 890.
The difference between these two numbers is 99009.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 90
<strong>Output:</strong> 99
<strong>Explanation:</strong>
The maximum value that can be returned by the function is 99 (if 0 is replaced by 9) and the minimum value that can be returned by the function is 0 (if 9 is replaced by 0).
Thus, we return 99.
</pre>

#### Constraints:
* <code>1 <= num <= 10<sup>8</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minMaxDifference(self, num: int) -> int:
        num = str(num)
        maxnum = int(num)
        minnum = int(num.replace(num[0], '0'))

        for i in range(len(num)):
            if num[i] != '9':
                maxnum = int(num.replace(num[i], '9'))
                break

        return maxnum - minnum
```
