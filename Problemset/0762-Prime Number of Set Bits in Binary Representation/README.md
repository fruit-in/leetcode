# 762. Prime Number of Set Bits in Binary Representation
Given two integers ```L``` and ```R```, find the count of numbers in the range ```[L, R]``` (inclusive) having a prime number of set bits in their binary representation.

(Recall that the number of set bits an integer has is the number of ```1```s present when written in binary. For example, ```21``` written in binary is ```10101``` which has 3 set bits. Also, 1 is not a prime.)

#### Example 1:
<pre>
<strong>Input:</strong> L = 6, R = 10
<strong>Output:</strong> 4
<strong>Explanation:</strong>
6 -> 110 (2 set bits, 2 is prime)
7 -> 111 (3 set bits, 3 is prime)
9 -> 1001 (2 set bits , 2 is prime)
10->1010 (2 set bits , 2 is prime)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> L = 10, R = 15
<strong>Output:</strong> 5
<strong>Explanation:</strong>
10 -> 1010 (2 set bits, 2 is prime)
11 -> 1011 (3 set bits, 3 is prime)
12 -> 1100 (2 set bits, 2 is prime)
13 -> 1101 (3 set bits, 3 is prime)
14 -> 1110 (3 set bits, 3 is prime)
15 -> 1111 (4 set bits, 4 is not prime)
</pre>

#### Note:
1. ```L, R``` will be integers ```L <= R``` in the range ```[1, 10^6]```.
2. ```R - L``` will be at most 10000.

## Solutions (Python)

### 1. Count
```Python3
class Solution:
    def countPrimeSetBits(self, L: int, R: int) -> int:
        cnt = 0

        for i in range(L, R + 1):
            if bin(i).count('1') in {2, 3, 5, 7, 11, 13, 17, 19}:
                cnt += 1

        return cnt
```
