# 423. 从英文中重建数字
给定一个**非空**字符串，其中包含字母顺序打乱的英文单词表示的数字```0-9```。按升序输出原始的数字。

#### 注意:
1. 输入只包含小写英文字母。
2. 输入保证合法并可以转换为原始的数字，这意味着像 "abc" 或 "zerone" 的输入是不允许的。
3. 输入字符串的长度小于 50,000。

#### 示例 1:
<pre>
<strong>输入:</strong> "owoztneoer"
<strong>输出:</strong> "012"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "fviefuro"
<strong>输出:</strong> "45"
</pre>

## 题解 (Rust)

### 1. 计数
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
