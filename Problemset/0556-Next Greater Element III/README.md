# 556. Next Greater Element III
Given a positive integer `n`, find *the smallest integer which has exactly the same digits existing in the integer* `n` *and is greater in value than* `n`. If no such positive integer exists, return `-1`.

**Note** that the returned integer should fit in **32-bit integer**, if there is a valid answer but it does not fit in **32-bit integer**, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 12
<strong>Output:</strong> 21
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 21
<strong>Output:</strong> -1
</pre>

#### Constraints:
* <code>1 <= n <= 2<sup>31</sup> - 1</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = n.to_string().into_bytes();
        let mut indices = vec![];
        let mut exist = false;
        let mut ret: i32 = 0;

        for i in (0..digits.len()).rev() {
            match indices.binary_search_by_key(&digits[i], |&j| digits[j]) {
                Ok(k) if k != indices.len() - 1 => {
                    digits.swap(i, indices[k + 1]);
                    exist = true;
                }
                Ok(k) => (),
                Err(k) if k != indices.len() => {
                    digits.swap(i, indices[k]);
                    exist = true;
                }
                Err(k) => indices.push(i),
            }

            if exist {
                let mut tmp = digits.split_off(i + 1);
                tmp.sort_unstable();
                digits.append(&mut tmp);
                break;
            }
        }

        for i in 0..digits.len() {
            let (tmp, overflow) = ret.overflowing_mul(10);
            ret = tmp;
            exist &= !overflow;
            let (tmp, overflow) = ret.overflowing_add((digits[i] - b'0') as i32);
            ret = tmp;
            exist &= !overflow;

            if !exist {
                return -1;
            }
        }

        ret
    }
}
```
