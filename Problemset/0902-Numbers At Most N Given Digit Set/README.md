# 902. Numbers At Most N Given Digit Set
Given an array of `digits` which is sorted in **non-decreasing** order. You can write numbers using each `digits[i]` as many times as we want. For example, if `digits = ['1','3','5']`, we may write numbers such as `'13'`, `'551'`, and `'1351315'`.

Return *the number of positive integers that can be generated* that are less than or equal to a given integer `n`.

#### Example 1:
<pre>
<strong>Input:</strong> digits = ["1","3","5","7"], n = 100
<strong>Output:</strong> 20
<strong>Explanation:</strong>
The 20 numbers that can be written are:
1, 3, 5, 7, 11, 13, 15, 17, 31, 33, 35, 37, 51, 53, 55, 57, 71, 73, 75, 77.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> digits = ["1","4","9"], n = 1000000000
<strong>Output:</strong> 29523
<strong>Explanation:</strong>
We can write 3 one digit numbers, 9 two digit numbers, 27 three digit numbers,
81 four digit numbers, 243 five digit numbers, 729 six digit numbers,
2187 seven digit numbers, 6561 eight digit numbers, and 19683 nine digit numbers.
In total, this is 29523 integers that can be written using the digits array.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> digits = ["7"], n = 8
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= digits.length <= 9`
* `digits[i].length == 1`
* `digits[i]` is a digit from `'1'` to `'9'`.
* All the values in `digits` are **unique**.
* `digits` is sorted in **non-decreasing** order.
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let digits = digits
            .iter()
            .map(|d| d.parse::<u8>().unwrap() + b'0')
            .collect::<Vec<_>>();
        let digitsn = n.to_string().into_bytes();
        let q = digits.len();
        let n = digitsn.len();
        let mut ret = n;

        if q > 1 {
            ret = q * (q.pow(n as u32) - 1) / (q - 1);

            for i in 0..n {
                match digits.binary_search(&digitsn[i]) {
                    Ok(j) => ret -= q.pow((n - i - 1) as u32) * (q - j - 1),
                    Err(j) => {
                        ret -= q.pow((n - i - 1) as u32) * (q - j);
                        break;
                    }
                }
            }
        } else if vec![digits[0]; n] > digitsn {
            ret -= 1;
        }

        ret as i32
    }
}
```
