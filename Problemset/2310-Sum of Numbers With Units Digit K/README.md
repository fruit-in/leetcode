# 2310. Sum of Numbers With Units Digit K
Given two integers `num` and `k`, consider a set of positive integers with the following properties:

* The units digit of each integer is `k`.
* The sum of the integers is `num`.

Return *the **minimum** possible size of such a set, or* `-1` *if no such set exists*.

Note:

* The set can contain multiple instances of the same integer, and the sum of an empty set is considered `0`.
* The **units digit** of a number is the rightmost digit of the number.

#### Example 1:
<pre>
<strong>Input:</strong> num = 58, k = 9
<strong>Output:</strong> 2
<strong>Explanation:</strong>
One valid set is [9,49], as the sum is 58 and each integer has a units digit of 9.
Another valid set is [19,39].
It can be shown that 2 is the minimum possible size of a valid set.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 37, k = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is not possible to obtain a sum of 37 using only integers that have a units digit of 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = 0, k = 7
<strong>Output:</strong> 0
<strong>Explanation:</strong> The sum of an empty set is considered 0.
</pre>

#### Constraints:
* `0 <= num <= 3000`
* `0 <= k <= 9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }

        let mut sum = k;

        for i in 1..=10 {
            if sum <= num && sum % 10 == num % 10 {
                return i;
            }
            sum += k;
        }

        -1
    }
}
```
