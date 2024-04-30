# 842. Split Array into Fibonacci Sequence
You are given a string of digits `num`, such as `"123456579"`. We can split it into a Fibonacci-like sequence `[123, 456, 579]`.

Formally, a **Fibonacci-like** sequence is a list `f` of non-negative integers such that:

* <code>0 <= f[i] < 2<sup>31</sup></code>, (that is, each integer fits in a **32-bit** signed integer type),
* `f.length >= 3`, and
* `f[i] + f[i + 1] == f[i + 2]` for all `0 <= i < f.length - 2`.

Note that when splitting the string into pieces, each piece must not have extra leading zeroes, except if the piece is the number `0` itself.

Return any Fibonacci-like sequence split from `num`, or return `[]` if it cannot be done.

#### Example 1:
<pre>
<strong>Input:</strong> num = "1101111"
<strong>Output:</strong> [11,0,11,11]
<strong>Explanation:</strong> The output [110, 1, 111] would also be accepted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "112358130"
<strong>Output:</strong> []
<strong>Explanation:</strong> The task is impossible.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = "0123"
<strong>Output:</strong> []
<strong>Explanation:</strong> Leading zeroes are not allowed, so "01", "2", "3" is not valid.
</pre>

#### Constraints:
* `1 <= num.length <= 200`
* `num` contains only digits.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def splitIntoFibonacci(self, num: str) -> List[int]:
        for i in range(1, min(11, (len(num) + 1) // 2)):
            if i > 1 and num[0] == '0':
                break

            for j in range(i + 1, min(i + 11, len(num))):
                if j - i > 1 and num[i] == '0':
                    break

                seq = [int(num[:i]), int(num[i:j])]
                k = j

                while k < len(num):
                    size = len(str(seq[-2] + seq[-1]))
                    seq.append(int(num[k:k + size]))
                    k += size

                    if seq[-1] >= 2147483648 or seq[-3] + seq[-2] != seq[-1]:
                        seq = []
                        break

                if seq != []:
                    return seq

        return []
```
