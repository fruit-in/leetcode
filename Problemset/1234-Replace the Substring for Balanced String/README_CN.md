# 1234. 替换子串得到平衡字符串
有一个只含有 `'Q', 'W', 'E', 'R'` 四种字符，且长度为 `n` 的字符串。

假如在该字符串中，这四个字符都恰好出现 `n/4` 次，那么它就是一个「平衡字符串」。



给你一个这样的字符串 `s`，请通过「替换一个子串」的方式，使原字符串 `s` 变成一个「平衡字符串」。

你可以用和「待替换子串」长度相同的 **任何** 其他字符串来完成替换。

请返回待替换子串的最小可能长度。

如果原字符串自身就是一个平衡字符串，则返回 `0`。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "QWER"
<strong>输出:</strong> 0
<strong>解释:</strong> s 已经是平衡的了。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "QQWE"
<strong>输出:</strong> 1
<strong>解释:</strong> 我们需要把一个 'Q' 替换成 'R'，这样得到的 "RQWE" (或 "QRWE") 是平衡的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "QQQW"
<strong>输出:</strong> 2
<strong>解释:</strong> 我们可以把前面的 "QQ" 替换成 "ER"。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "QQQQ"
<strong>输出:</strong> 3
<strong>解释:</strong> 我们可以替换后 3 个 'Q'，使 s = "QWER"。
</pre>

#### 提示:
* `1 <= s.length <= 10^5`
* `s.length` 是 `4` 的倍数
* `s` 中只含有 `'Q'`, `'W'`, `'E'`, `'R'` 四种字符

## 题解 (Ruby)

### 1. 双指针
```Ruby
# @param {String} s
# @return {Integer}
def balanced_string(s)
  count = {}
  count.default = 0
  l = 0
  ret = s.size

  s.each_char { |c| count[c] += 1 }

  (0..s.size).each do |r|
    while l <= r && 4 * count.values.max - count.values.sum <= r - l
      ret = [ret, r - l].min
      count[s[[l, s.size - 1].min]] += 1
      l += 1
    end
    count[s[[r, s.size - 1].min]] -= 1
  end

  ret
end
```

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 4];
        let mut l = 0;
        let mut ret = s.len();

        for c in s {
            match c {
                b'Q' => count[0] += 1,
                b'W' => count[1] += 1,
                b'E' => count[2] += 1,
                _ => count[3] += 1,
            }
        }

        for r in 0..=s.len() {
            while l <= r && 4 * count.iter().max().unwrap() - count.iter().sum::<usize>() <= r - l {
                ret = ret.min(r - l);
                match s[l.min(s.len() - 1)] {
                    b'Q' => count[0] += 1,
                    b'W' => count[1] += 1,
                    b'E' => count[2] += 1,
                    _ => count[3] += 1,
                }
                l += 1;
            }
            match s[r.min(s.len() - 1)] {
                b'Q' => count[0] -= 1,
                b'W' => count[1] -= 1,
                b'E' => count[2] -= 1,
                _ => count[3] -= 1,
            }
        }

        ret as i32
    }
}
```
