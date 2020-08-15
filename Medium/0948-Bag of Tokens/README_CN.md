# 948. 令牌放置
你的初始能量为 `P`，初始分数为 `0`，只有一包令牌。

令牌的值为 `token[i]`，每个令牌最多只能使用一次，可能的两种使用方法如下：
* 如果你至少有 `token[i]` 点能量，可以将令牌置为正面朝上，失去 `token[i]` 点能量，并得到 `1` 分。
* 如果我们至少有 `1` 分，可以将令牌置为反面朝上，获得 `token[i]` 点能量，并失去 `1` 分。

在使用任意数量的令牌后，返回我们可以得到的最大分数。

#### 示例 1:
<pre>
<strong>输入:</strong> tokens = [100], P = 50
<strong>输出:</strong> 0
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tokens = [100,200], P = 150
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> tokens = [100,200,300,400], P = 200
<strong>输出:</strong> 2
</pre>

#### 提示:
1. `tokens.length <= 1000`
2. `0 <= tokens[i] < 10000`
3. `0 <= P < 10000`

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer[]} tokens
# @param {Integer} p
# @return {Integer}
def bag_of_tokens_score(tokens, p)
    return 0 if tokens.length == 0

    i, j = 0, tokens.length - 1
    score = 0
    ret = 0
    tokens.sort!

    while i <= j
        if p >= tokens[i]
            p -= tokens[i]
            score += 1
            ret = [ret, score].max
            i += 1
        elsif score > 0
            p += tokens[j]
            score -= 1
            j -= 1
        else
            break
        end
    end

    return ret
end
```

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }

        let mut tokens = tokens;
        let mut p = p;
        let mut i = 0;
        let mut j = tokens.len() - 1;
        let mut score = 0;
        let mut ret = 0;
        tokens.sort_unstable();

        while i <= j {
            if p >= tokens[i] {
                p -= tokens[i];
                score += 1;
                ret = ret.max(score);
                i += 1;
            } else if score > 0 {
                p += tokens[j];
                score -= 1;
                j -= 1;
            } else {
                break;
            }
        }

        ret
    }
}
```
