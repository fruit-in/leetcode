# 948. Bag of Tokens
You have an initial power `P`, an initial score of `0` points, and a bag of tokens.

Each token can be used at most once, has a value `token[i]`, and has potentially two ways to use it.
* If we have at least `token[i]` power, we may play the token face up, losing `token[i]` power, and gaining `1` point.
* If we have at least `1` point, we may play the token face down, gaining `token[i]` power, and losing `1` point.

Return the largest number of points we can have after playing any number of tokens.

#### Example 1:
<pre>
<strong>Input:</strong> tokens = [100], P = 50
<strong>Output:</strong> 0
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tokens = [100,200], P = 150
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> tokens = [100,200,300,400], P = 200
<strong>Output:</strong> 2
</pre>

#### Note:
1. `tokens.length <= 1000`
2. `0 <= tokens[i] < 10000`
3. `0 <= P < 10000`

## Solutions (Ruby)

### 1. Greedy
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

## Solutions (Rust)

### 1. Greedy
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
