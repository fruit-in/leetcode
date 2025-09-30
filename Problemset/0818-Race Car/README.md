# 818. Race Car
Your car starts at position `0` and speed `+1` on an infinite number line. Your car can go into negative positions. Your car drives automatically according to a sequence of instructions `'A'` (accelerate) and `'R'` (reverse):
* When you get an instruction `'A'`, your car does the following:
    * `position += speed`
    * `speed *= 2`
* When you get an instruction `'R'`, your car does the following:
    * If your speed is positive then `speed = -1`
    * otherwise `speed = 1`
    * Your position stays the same.

For example, after commands `"AAR"`, your car goes to positions `0 --> 1 --> 3 --> 3`, and your speed goes to `1 --> 2 --> 4 --> -1`.

Given a target position `target`, return *the length of the shortest sequence of instructions to get there*.

#### Example 1:
<pre>
<strong>Input:</strong> target = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The shortest instruction sequence is "AA".
Your position goes from 0 --> 1 --> 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = 6
<strong>Output:</strong> 5
<strong>Explanation:</strong>
The shortest instruction sequence is "AAARA".
Your position goes from 0 --> 1 --> 3 --> 7 --> 7 --> 6.
</pre>

#### Constraints:
* <code>1 <= target <= 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    aseqpositions = [(1 << i) - 1 for i in range(15)]

    @cache
    def racecar(self, target: int, reverse: bool = False) -> int:
        ret = inf

        for i in range(15):
            if not reverse and self.aseqpositions[i] == target:
                return i

            for j in range(i - 1, -1, -1):
                position = self.aseqpositions[i] - self.aseqpositions[j]

                if position < target:
                    ret = min(ret, i + j + 2 +
                              self.racecar(target - position, reverse))
                elif position == target:
                    ret = min(ret, i + j + 1)
                elif position - target < target:
                    ret = min(ret, i + j + 2 +
                              self.racecar(position - target, not reverse))
                else:
                    break

        return ret
```
