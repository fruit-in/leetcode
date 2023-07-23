# 1138. 字母板上的路径
我们从一块字母板上的位置 `(0, 0)` 出发，该坐标对应的字符为 `board[0][0]`。

在本题里，字母板为`board = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"]`，如下所示。

![](https://assets.leetcode.com/uploads/2019/07/28/azboard.png)

我们可以按下面的指令规则行动：
* 如果方格存在，`'U'` 意味着将我们的位置上移一行；
* 如果方格存在，`'D'` 意味着将我们的位置下移一行；
* 如果方格存在，`'L'` 意味着将我们的位置左移一列；
* 如果方格存在，`'R'` 意味着将我们的位置右移一列；
* `'!'` 会把在我们当前位置 `(r, c)` 的字符 `board[r][c]` 添加到答案中。

（注意，字母板上只存在有字母的位置。）

返回指令序列，用最小的行动次数让答案和目标 `target` 相同。你可以返回任何达成目标的路径。

#### 示例 1:
<pre>
<strong>输入:</strong> target = "leet"
<strong>输出:</strong> "DDR!UURRR!!DDD!"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = "code"
<strong>输出:</strong> "RR!DDRR!UUL!R!"
</pre>

#### 提示:
* `1 <= target.length <= 100`
* `target` 仅含有小写英文字母。

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
