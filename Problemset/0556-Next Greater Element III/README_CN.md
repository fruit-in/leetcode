# 556. 下一个更大元素 III
给你一个正整数 `n` ，请你找出符合条件的最小整数，其由重新排列 `n` 中存在的每位数字组成，并且其值大于 `n` 。如果不存在这样的正整数，则返回 `-1` 。

**注意** ，返回的整数应当是一个 **32 位整数** ，如果存在满足题意的答案，但不是 **32 位整数** ，同样返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 12
<strong>输出:</strong> 21
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 21
<strong>输出:</strong> -1
</pre>

#### 提示:
* <code>1 <= n <= 2<sup>31</sup> - 1</code>

## 题解 (Rust)

### 1. 题解
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
