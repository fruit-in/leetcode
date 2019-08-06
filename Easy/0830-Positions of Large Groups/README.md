# 830. Positions of Large Groups
In a string <code>S</code> of lowercase letters, these letters form consecutive groups of the same character.

For example, a string like <code>S = "abbxxxxzyy"</code> has the groups <code>"a"</code>, <code>"bb"</code>, <code>"xxxx"</code>, <code>"z"</code> and <code>"yy"</code>.

Call a group *large* if it has 3 or more characters.  We would like the starting and ending positions of every large group.

The final answer should be in lexicographic order.

#### Example 1:
<pre>
<strong>Input:</strong> "abbxxxxzzy"
<strong>Output:</strong> [[3,6]]
<strong>Explanation:</strong> "xxxx" is the single large group with starting  3 and ending positions 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "abc"
<strong>Output:</strong> []
<strong>Explanation:</strong> We have "a","b" and "c" but no large group.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "abcdddeeeeaabbbcd"
<strong>Output:</strong> [[3,5],[6,9],[12,14]]
</pre>

**Note:** <code>1 <= S.length <= 1000</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut groups = Vec::new();
        let mut start = 0;
        let mut count = 0;
        let mut cur_ch = ' ';
        for ch in s.chars() {
            if ch == cur_ch {
                count += 1;
            } else if ch != cur_ch {
                if count >= 3 {
                    groups.push(vec![start, start + count - 1]);
                }
                start += count;
                count = 1;
                cur_ch = ch;
            }
        }
        if count >= 3 {
            groups.push(vec![start, start + count - 1]);
        }
        groups
    }
}
```
