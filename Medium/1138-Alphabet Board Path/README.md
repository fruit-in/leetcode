# 1138. Alphabet Board Path
On an alphabet board, we start at position `(0, 0)`, corresponding to character `board[0][0]`.

Here, `board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"]`, as shown in the diagram below.

![](https://assets.leetcode.com/uploads/2019/07/28/azboard.png)

We may make the following moves:
* `'U'` moves our position up one row, if the position exists on the board;
* `'D'` moves our position down one row, if the position exists on the board;
* `'L'` moves our position left one column, if the position exists on the board;
* `'R'` moves our position right one column, if the position exists on the board;
* `'!'` adds the character `board[r][c]` at our current position `(r, c)` to the answer.

(Here, the only positions that exist on the board are positions with letters on them.)

Return a sequence of moves that makes our answer equal to `target` in the minimum number of moves.  You may return any path that does so.

#### Example 1:
<pre>
<strong>Input:</strong> target = "leet"
<strong>Output:</strong> "DDR!UURRR!!DDD!"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = "code"
<strong>Output:</strong> "RR!DDRR!UUL!R!"
</pre>

#### Constraints:
* `1 <= target.length <= 100`
* `target` consists only of English lowercase letters.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} target
# @return {String}
def alphabet_board_path(target)
  s = [0, 0]
  ret = ''

  target.each_byte do |c|
    t = [(c - 97) / 5, (c - 97) % 5]
    ret += path_to(s, t) + '!'
    s = t
  end

  ret
end

# @param {Integer[]} start
# @param {Integer[]} target
# @return {String}
def path_to(start, target)
  if start == target
    ''
  elsif start == [5, 0]
    'U' + path_to([4, 0], target)
  elsif target == [5, 0]
    path_to(start, [4, 0]) + 'D'
  else
    v_dir = start[0] > target[0] ? 'U' : 'D'
    h_dir = start[1] > target[1] ? 'L' : 'R'
    v_dir * (start[0] - target[0]).abs + h_dir * (start[1] - target[1]).abs
  end
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut s = (0, 0);
        let mut ret = String::new();

        for c in target.bytes() {
            let t = ((c - b'a') / 5, (c - b'a') % 5);
            ret = ret + &Self::path_to(s, t) + "!";
            s = t;
        }

        ret
    }

    fn path_to(start: (u8, u8), target: (u8, u8)) -> String {
        if start == target {
            String::new()
        } else if start == (5, 0) {
            "U".to_string() + &Self::path_to((4, 0), target)
        } else if target == (5, 0) {
            Self::path_to(start, (4, 0)) + "D"
        } else {
            let mut moves = String::new();
            if start.0 > target.0 {
                moves += &"U".repeat((start.0 - target.0) as usize)
            } else {
                moves += &"D".repeat((target.0 - start.0) as usize)
            }
            if start.1 > target.1 {
                moves += &"L".repeat((start.1 - target.1) as usize);
            } else {
                moves += &"R".repeat((target.1 - start.1) as usize);
            }
            moves
        }
    }
}
```
