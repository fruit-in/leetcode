# 2231. Largest Number After Digit Swaps by Parity
You are given a positive integer `num`. You may swap any two digits of `num` that have the same **parity** (i.e. both odd digits or both even digits).

Return *the **largest** possible value of* `num` *after **any** number of swaps*.

#### Example 1:
<pre>
<strong>Input:</strong> num = 1234
<strong>Output:</strong> 3412
<strong>Explanation:</strong> Swap the digit 3 with the digit 1, this results in the number 3214.
Swap the digit 2 with the digit 4, this results in the number 3412.
Note that there may be other sequences of swaps but it can be shown that 3412 is the largest possible number.
Also note that we may not swap the digit 4 with the digit 1 since they are of different parities.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 65875
<strong>Output:</strong> 87655
<strong>Explanation:</strong> Swap the digit 8 with the digit 6, this results in the number 85675.
Swap the first digit 5 with the digit 7, this results in the number 87655.
Note that there may be other sequences of swaps but it can be shown that 87655 is the largest possible number.
</pre>

#### Constraints:
* <code>1 <= num <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut num = num;
        let mut odd_digits = vec![];
        let mut even_digits = vec![];
        let mut is_odds = vec![];
        let mut ret = 0;

        while num > 0 {
            if num % 2 == 1 {
                odd_digits.push(num % 10);
                is_odds.push(true);
            } else {
                even_digits.push(num % 10);
                is_odds.push(false);
            }
            num /= 10;
        }

        odd_digits.sort_unstable();
        even_digits.sort_unstable();

        while let Some(is_odd) = is_odds.pop() {
            if is_odd {
                ret = ret * 10 + odd_digits.pop().unwrap();
            } else {
                ret = ret * 10 + even_digits.pop().unwrap();
            }
        }

        ret
    }
}
```
