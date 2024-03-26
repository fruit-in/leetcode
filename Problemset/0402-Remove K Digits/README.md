# 402. Remove K Digits
Given string num representing a non-negative integer `num`, and an integer `k`, return *the smallest possible integer after removing* `k` *digits from* `num`.

#### Example 1:
<pre>
<strong>Input:</strong> num = "1432219", k = 3
<strong>Output:</strong> "1219"
<strong>Explanation:</strong> Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "10200", k = 1
<strong>Output:</strong> "200"
<strong>Explanation:</strong> Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = "10", k = 2
<strong>Output:</strong> "0"
<strong>Explanation:</strong> Remove all the digits from the number and it is left with nothing which is 0.
</pre>

#### Constraints:
* <code>1 <= k <= num.length <= 10<sup>5</sup></code>
* `num` consists of only digits.
* `num` does not have any leading zeros except for the zero itself.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let keep = num.len() - k as usize;
        let mut stack = vec![];

        for (i, digit) in num.bytes().enumerate() {
            while *stack.last().unwrap_or(&b'0') > digit && num.len() - i + stack.len() > keep {
                stack.pop();
            }

            if stack.len() < keep {
                stack.push(digit);
            }
        }

        if stack.iter().all(|&digit| digit == b'0') {
            return "0".to_string();
        }

        String::from_utf8(stack)
            .unwrap()
            .trim_start_matches('0')
            .to_string()
    }
}
```
