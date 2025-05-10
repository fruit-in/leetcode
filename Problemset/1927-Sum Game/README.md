# 1927. Sum Game
Alice and Bob take turns playing a game, with **Alice starting first**.

You are given a string `num` of **even length** consisting of digits and `'?'` characters. On each turn, a player will do the following if there is still at least one `'?'` in `num`:
1. Choose an index `i` where `num[i] == '?'`.
2. Replace `num[i]` with any digit between `'0'` and `'9'`.

The game ends when there are no more `'?'` characters in `num`.

For Bob to win, the sum of the digits in the first half of `num` must be **equal** to the sum of the digits in the second half. For Alice to win, the sums must **not be equal**.

* For example, if the game ended with `num = "243801"`, then Bob wins because `2+4+3 = 8+0+1`. If the game ended with `num = "243803"`, then Alice wins because `2+4+3 != 8+0+3`.

Assuming Alice and Bob play **optimally**, return `true` *if Alice will win and* `false` *if Bob will win*.

#### Example 1:
<pre>
<strong>Input:</strong> num = "5023"
<strong>Output:</strong> false
<strong>Explanation:</strong> There are no moves to be made.
The sum of the first half is equal to the sum of the second half: 5 + 0 = 2 + 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "25??"
<strong>Output:</strong> true
<strong>Explanation:</strong> Alice can replace one of the '?'s with '9' and it will be impossible for Bob to make the sums equal.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = "?3295???"
<strong>Output:</strong> false
<strong>Explanation:</strong> It can be proven that Bob will always win. One possible outcome is:
- Alice replaces the first '?' with '9'. num = "93295???".
- Bob replaces one of the '?' in the right half with '9'. num = "932959??".
- Alice replaces one of the '?' in the right half with '2'. num = "9329592?".
- Bob replaces the last '?' in the right half with '7'. num = "93295927".
Bob wins because 9 + 3 + 2 + 9 = 5 + 9 + 2 + 7.
</pre>

#### Constraints:
* <code>2 <= num.length <= 10<sup>5</sup></code>
* `num.length` is **even**.
* `num` consists of only digits and `'?'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_game(num: String) -> bool {
        let num = num.as_bytes();
        let mut diff = 0;
        let mut diff_mask = 0;

        for i in 0..num.len() / 2 {
            if num[i] != b'?' {
                diff += (num[i] - b'0') as i32;
            } else {
                diff_mask -= 1;
            }
        }
        for i in num.len() / 2..num.len() {
            if num[i] != b'?' {
                diff -= (num[i] - b'0') as i32;
            } else {
                diff_mask += 1;
            }
        }

        diff_mask % 2 != 0 || diff_mask / 2 * 9 != diff
    }
}
```
