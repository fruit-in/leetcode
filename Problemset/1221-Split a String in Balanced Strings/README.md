# 1221. Split a String in Balanced Strings
*Balanced* strings are those who have equal quantity of 'L' and 'R' characters.

Given a balanced string ```s``` split it in the maximum amount of balanced strings.

Return the maximum amount of splitted balanced strings.

#### Example 1:
<pre>
<strong>Input:</strong> s = "RLRRLLRLRL"
<strong>Output:</strong> 4
<strong>Explanation:</strong> s can be split into "RL", "RRLL", "RL", "RL", each substring contains same number of 'L' and 'R'.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "RLLLLRRRLR"
<strong>Output:</strong> 3
<strong>Explanation:</strong> s can be split into "RL", "LLLRRR", "LR", each substring contains same number of 'L' and 'R'.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "LLLLRRRR"
<strong>Output:</strong> 1
<strong>Explanation:</strong> s can be split into "LLLLRRRR".
</pre>

#### Constraints:
* ```1 <= s.length <= 1000```
* ```s[i] = 'L' or 'R'```

## Solutions (Ruby)

### 1. Greedy
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

## Solutions (Rust)

### 1. Greedy
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
