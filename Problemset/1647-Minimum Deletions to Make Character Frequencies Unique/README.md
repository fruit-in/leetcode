# 1647. Minimum Deletions to Make Character Frequencies Unique
A string `s` is called **good** if there are no two different characters in `s` that have the same **frequency**.

Given a string `s`, return *the **minimum** number of characters you need to delete to make* `s` ***good***.

The **frequency** of a character in a string is the number of times it appears in the string. For example, in the string `"aab"`, the **frequency** of `'a'` is `2`, while the **frequency** of `'b'` is `1`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aab"
<strong>Output:</strong> 0
<strong>Explanation:</strong> s is already good.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aaabbbcc"
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can delete two 'b's resulting in the good string "aaabcc".
Another way it to delete one 'b' and one 'c' resulting in the good string "aaabbc".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "ceabaacb"
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can delete both 'c's resulting in the good string "eabaab".
Note that we only care about characters that are still in the string at the end (i.e. frequency of 0 is ignored).
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` contains only lowercase English letters.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} s
# @return {Integer}
def min_deletions(s)
  counter = [0] * 26
  ret = 0

  s.each_byte { |c| counter[c - 97] += 1 }
  counter.sort!

  (25..1).step(-1).each do |i|
    if counter[i] <= counter[i - 1]
      ret += [counter[i - 1] - counter[i] + 1, counter[i - 1]].min
      counter[i - 1] = [counter[i] - 1, 0].max
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counter = vec![0; 26];
        let mut ret = 0;

        for c in s.bytes() {
            counter[(c - b'a') as usize] += 1;
        }
        counter.sort_unstable();

        for i in (1..26).rev() {
            if counter[i] <= counter[i - 1] {
                ret += (counter[i - 1] - counter[i] + 1).min(counter[i - 1]);
                counter[i - 1] = (counter[i] - 1).max(0);
            }
        }

        ret
    }
}
```
