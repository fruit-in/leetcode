# 1404. Number of Steps to Reduce a Number in Binary Representation to One
Given the binary representation of an integer as a string `s`, return *the number of steps to reduce it to* `1` *under the following rules*:

* If the current number is even, you have to divide it by `2`.

* If the current number is odd, you have to add `1` to it.

It is guaranteed that you can always reach one for all test cases.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1101"
<strong>Output:</strong> 6
<strong>Explanation:</strong> "1101" corressponds to number 13 in their decimal representation.
Step 1) 13 is odd, add 1 and obtain 14.
Step 2) 14 is even, divide by 2 and obtain 7.
Step 3) 7 is odd, add 1 and obtain 8.
Step 4) 8 is even, divide by 2 and obtain 4.
Step 5) 4 is even, divide by 2 and obtain 2.
Step 6) 2 is even, divide by 2 and obtain 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "10"
<strong>Output:</strong> 1
<strong>Explanation:</strong> "10" corressponds to number 2 in their decimal representation.
Step 1) 2 is even, divide by 2 and obtain 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1"
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= s.length <= 500`
* `s` consists of characters '0' or '1'
* `s[0] == '1'`

## Solutions (Rust)

### 1. Solution1
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut bits = s.chars().map(|c| c == '1').collect::<VecDeque<_>>();
        let mut ret = 0;

        while bits.len() > 1 {
            if *bits.back().unwrap() {
                let mut carry = true;

                for i in (0..bits.len()).rev() {
                    if carry {
                        bits[i] = !bits[i];
                        carry = !bits[i];
                    } else {
                        break;
                    }
                }

                if carry {
                    bits.push_front(true);
                }
            } else {
                bits.pop_back();
            }

            ret += 1
        }

        ret
    }
}
```

### 2. Solution2
```Rust
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut carry = false;
        let mut ret = s.len() as i32 - 1;

        for bit in s.chars().rev().take(s.len() - 1) {
            if (bit == '1') ^ carry {
                carry = true;
                ret += 1;
            }
        }

        ret + carry as i32
    }
}
```
