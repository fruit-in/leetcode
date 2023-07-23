# 1221. 分割平衡字符串
在一个「平衡字符串」中，'L' 和 'R' 字符的数量是相同的。

给出一个平衡字符串 ```s```，请你将它分割成尽可能多的平衡字符串。

返回可以通过分割得到的平衡字符串的最大数量。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "RLRRLLRLRL"
<strong>输出:</strong> 4
<strong>解释:</strong> s 可以分割为 "RL", "RRLL", "RL", "RL", 每个子字符串中都包含相同数量的 'L' 和 'R'。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "RLLLLRRRLR"
<strong>输出:</strong> 3
<strong>解释:</strong> s 可以分割为 "RL", "LLLRRR", "LR", 每个子字符串中都包含相同数量的 'L' 和 'R'。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "LLLLRRRR"
<strong>输出:</strong> 1
<strong>解释:</strong> s 只能保持原样 "LLLLRRRR".
</pre>

#### 提示:
* ```1 <= s.length <= 1000```
* ```s[i] = 'L' 或 'R'```

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {String} s
# @return {Integer}
def balanced_string_split(s)
    cnt = 0
    ret = 0

    for ch in s.chars
        if ch == 'R'
            cnt += 1
        elsif ch == 'L'
            cnt -= 1
        end

        if cnt == 0
            ret += 1
        end
    end

    return ret
end
```

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut amt = 0;
        let mut cnt = 0;

        for ch in s.chars() {
            if ch == 'R' {
                cnt += 1;
            } else if ch == 'L' {
                cnt -= 1;
            }

            if cnt == 0 {
                amt += 1;
            }
        }

        amt
    }
}
```
