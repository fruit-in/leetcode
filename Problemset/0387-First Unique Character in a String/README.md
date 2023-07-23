# 387. First Unique Character in a String
Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.

#### Example:
```
s = "leetcode"
return 0.

s = "loveleetcode",
return 2.
```

**Note:** You may assume the string contain only lowercase letters.

## Solutions (Ruby)

### 1. Count
```Ruby
# @param {String} s
# @return {Integer}
def first_uniq_char(s)
    cnt = [0] * 26

    for i in 0...s.length
        cnt[s[i].ord - 'a'.ord] += 1
    end

    for i in 0...s.length
        return i if cnt[s[i].ord - 'a'.ord] == 1
    end

    return -1
end
```

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnt = [0; 26];

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        for (i, ch) in s.bytes().enumerate() {
            if cnt[(ch - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}
```
