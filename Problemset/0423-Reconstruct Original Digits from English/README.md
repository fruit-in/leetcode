# 423. Reconstruct Original Digits from English
Given a **non-empty** string containing an out-of-order English representation of digits ```0-9```, output the digits in ascending order.

#### Note:
1. Input contains only lowercase English letters.
2. Input is guaranteed to be valid and can be transformed to its original digits. That means invalid inputs such as "abc" or "zerone" are not permitted.
3. Input length is less than 50,000.

#### Example 1:
<pre>
<strong>Input:</strong> "owoztneoer"
<strong>Output:</strong> "012"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "fviefuro"
<strong>Output:</strong> "45"
</pre>

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut let_cnt = [0; 26];
        let mut dig_cnt = [0; 10];
        let mut ret = String::new();

        s.bytes().for_each(|ch| let_cnt[(ch - b'a') as usize] += 1);

        dig_cnt[0] = let_cnt[25];
        dig_cnt[2] = let_cnt[22];
        dig_cnt[4] = let_cnt[20];
        dig_cnt[6] = let_cnt[23];
        dig_cnt[8] = let_cnt[6];
        dig_cnt[1] = let_cnt[14] - dig_cnt[0] - dig_cnt[2] - dig_cnt[4];
        dig_cnt[3] = let_cnt[7] - dig_cnt[8];
        dig_cnt[5] = let_cnt[5] - dig_cnt[4];
        dig_cnt[7] = let_cnt[18] - dig_cnt[6];
        dig_cnt[9] = (let_cnt[13] - dig_cnt[7] - dig_cnt[1]) / 2;

        for n in 0..10 {
            for _ in 0..dig_cnt[n] {
                ret.push((n as u8 + b'0') as char);
            }
        }

        ret
    }
}
```
