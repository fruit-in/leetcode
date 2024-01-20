# 1363. Largest Multiple of Three
Given an array of digits `digits`, return *the largest multiple of **three** that can be formed by concatenating some of the given digits in **any order***. If there is no answer return an empty string.

Since the answer may not fit in an integer data type, return the answer as a string. Note that the returning answer must not contain unnecessary leading zeros.

#### Example 1:
<pre>
<strong>Input:</strong> digits = [8,1,9]
<strong>Output:</strong> "981"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> digits = [8,6,7,1,0]
<strong>Output:</strong> "8760"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> digits = [1]
<strong>Output:</strong> ""
</pre>

#### Constraints:
* <code>1 <= digits.length <= 10<sup>4</sup></code>
* `0 <= digits[i] <= 9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut count = [0; 10];
        let mut sum_rem = 0;
        let mut min_rem = [vec![], vec![], vec![]];
        let mut ret = vec![];

        for d in digits {
            count[d as usize] += 1;
            sum_rem = (sum_rem + d) % 3;
        }

        for d in 1..9 {
            if min_rem[d % 3].len() < 2 && count[d] > 0 {
                min_rem[d % 3].push(d);
                if min_rem[d % 3].len() < 2 && count[d] > 1 {
                    min_rem[d % 3].push(d);
                }
            }
        }

        if sum_rem > 0 {
            if !min_rem[sum_rem as usize].is_empty() {
                count[min_rem[sum_rem as usize][0]] -= 1;
            } else {
                count[min_rem[3 - sum_rem as usize][0]] -= 1;
                count[min_rem[3 - sum_rem as usize][1]] -= 1;
            }
        }

        for d in (0..10).rev() {
            while count[d] > 0 {
                count[d] -= 1;
                ret.push(d as u8 + b'0');
            }
        }

        if *ret.get(0).unwrap_or(&1) == b'0' {
            return 0.to_string();
        }

        String::from_utf8(ret).unwrap()
    }
}
```
