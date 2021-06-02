# 984. 不含 AAA 或 BBB 的字符串
给定两个整数 `A` 和 `B`，返回任意字符串 `S`，要求满足：
* `S` 的长度为 `A + B`，且正好包含 `A` 个 `'a'` 字母与 `B` 个 `'b'` 字母；
* 子串 `'aaa'` 没有出现在 `S` 中；
* 子串 `'bbb'` 没有出现在 `S` 中。

#### 示例 1:
<pre>
<strong>输入:</strong> A = 1, B = 2
<strong>输出:</strong> "abb"
<strong>解释:</strong> "abb", "bab" 和 "bba" 都是正确答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = 4, B = 1
<strong>输出:</strong> "aabaa"
</pre>

#### 提示:
1. `0 <= A <= 100`
2. `0 <= B <= 100`
3. 对于给定的 `A` 和 `B`，保证存在满足要求的 `S`。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} a
# @param {Integer} b
# @return {String}
def str_without3a3b(a, b)
  more_ch, less_ch = a > b ? %w[a b] : %w[b a]
  more_cnt = [a, b].max
  less_cnt = [a, b].min

  part0 = more_ch * 2 + less_ch
  part1 = more_ch + less_ch
  part2 = more_ch

  x = [more_cnt - less_cnt, less_cnt].min
  y = [2 * less_cnt - more_cnt, 0].max
  z = [more_cnt - 2 * less_cnt, 0].max

  part0 * x + part1 * y + part2 * z
end
```
