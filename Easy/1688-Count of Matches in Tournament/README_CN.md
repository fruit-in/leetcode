# 1688. 比赛中的配对次数
给你一个整数 `n` ，表示比赛中的队伍数。比赛遵循一种独特的赛制：
* 如果当前队伍数是 **偶数** ，那么每支队伍都会与另一支队伍配对。总共进行 `n / 2` 场比赛，且产生 `n / 2` 支队伍进入下一轮。
* 如果当前队伍数为 **奇数** ，那么将会随机轮空并晋级一支队伍，其余的队伍配对。总共进行 `(n - 1) / 2` 场比赛，且产生 `(n - 1) / 2 + 1` 支队伍进入下一轮。

返回在比赛中进行的配对次数，直到决出获胜队伍为止。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 7
<strong>输出:</strong> 6
<strong>解释:</strong> 比赛详情：
- 第 1 轮：队伍数 = 7 ，配对次数 = 3 ，4 支队伍晋级。
- 第 2 轮：队伍数 = 4 ，配对次数 = 2 ，2 支队伍晋级。
- 第 3 轮：队伍数 = 2 ，配对次数 = 1 ，决出 1 支获胜队伍。
总配对次数 = 3 + 2 + 1 = 6
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 14
<strong>输出:</strong> 13
<strong>解释:</strong> 比赛详情：
- 第 1 轮：队伍数 = 14 ，配对次数 = 7 ，7 支队伍晋级。
- 第 2 轮：队伍数 = 7 ，配对次数 = 3 ，4 支队伍晋级。
- 第 3 轮：队伍数 = 4 ，配对次数 = 2 ，2 支队伍晋级。
- 第 4 轮：队伍数 = 2 ，配对次数 = 1 ，决出 1 支获胜队伍。
总配对次数 = 7 + 3 + 2 + 1 = 13
</pre>

#### 提示:
* `1 <= n <= 200`

## 题解 (Ruby)

### 1. 模拟
```Ruby
# @param {Integer} n
# @return {Integer}
def number_of_matches(n)
  if n == 1
    0
  elsif n.even?
    n / 2 + number_of_matches(n / 2)
  else
    (n - 1) / 2 + number_of_matches((n - 1) / 2 + 1)
  end
end
```

## 题解 (Rust)

### 1. 模拟
```Rust
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        if n == 1 {
            0
        } else if n % 2 == 0 {
            n / 2 + Self::number_of_matches(n / 2)
        } else {
            (n - 1) / 2 + Self::number_of_matches((n - 1) / 2 + 1)
        }
    }
}
```
