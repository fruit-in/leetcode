# 752. 打开转盘锁
你有一个带有四个圆形拨轮的转盘锁。每个拨轮都有10个数字： `'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'` 。每个拨轮可以自由旋转：例如把 `'9'` 变为  `'0'`，`'0'` 变为 `'9'` 。每次旋转都只能旋转一个拨轮的一位数字。

锁的初始数字为 `'0000'` ，一个代表四个拨轮的数字的字符串。

列表 `deadends` 包含了一组死亡数字，一旦拨轮的数字和列表里的任何一个元素相同，这个锁将会被永久锁定，无法再被旋转。

字符串 `target` 代表可以解锁的数字，你需要给出最小的旋转次数，如果无论如何不能解锁，返回 -1。

#### 示例 1:
<pre>
<strong>输入:</strong> deadends = ["0201","0101","0102","1212","2002"], target = "0202"
<strong>输出:</strong> 6
<strong>解释:</strong>
可能的移动序列为 "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202"。
注意 "0000" -> "0001" -> "0002" -> "0102" -> "0202" 这样的序列是不能解锁的，
因为当拨动到 "0102" 时这个锁就会被锁定。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> deadends = ["8888"], target = "0009"
<strong>输出:</strong> 1
<strong>解释:</strong>
把最后一位反向旋转一次即可 "0000" -> "0009"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
<strong>输出:</strong> -1
<strong>解释:</strong>
无法旋转到目标数字且不被锁定。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> deadends = ["0000"], target = "8888"
<strong>输出:</strong> -1
</pre>

#### 提示:
1. 死亡列表 `deadends` 的长度范围为 `[1, 500]`。
2. 目标数字 `target` 不会在 `deadends` 之中。
3. 每个 `deadends` 和 `target` 中的字符串的数字会在 10,000 个可能的情况 `'0000'` 到 `'9999'` 中产生。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String[]} deadends
# @param {String} target
# @return {Integer}
def open_lock(deadends, target)
  deadends = deadends.map { |deadend| deadend.to_i }.to_set
  target = target.to_i
  deadends.add(target)
  states = Containers::Queue.new([[target, 0]])

  until states.empty?
    state, i = states.pop

    return i if state == 0

    (0..3).each do |j|
      [-1, 1].each do |k|
        new_state = [state / 1000, state % 1000 / 100, state % 100 / 10, state % 10]
        new_state[j] = (new_state[j] + k) % 10
        new_state = new_state[0] * 1000 + new_state[1] * 100 + new_state[2] * 10 + new_state[3]

        unless deadends.member?(new_state)
          deadends.add(new_state)
          states.push([new_state, i + 1])
        end
      end
    end
  end

  -1
end
```

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deadends = deadends
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<HashSet<_>>();
        let target = target.parse::<i32>().unwrap();
        deadends.insert(target);
        let mut states = VecDeque::new();
        states.push_back((target, 0));

        while let Some((state, i)) = states.pop_front() {
            if state == 0 {
                return i;
            }

            for j in 0..4 {
                for k in &[1, 9] {
                    let mut new_state = [
                        state / 1000,
                        state % 1000 / 100,
                        state % 100 / 10,
                        state % 10,
                    ];
                    new_state[j] = (new_state[j] + k) % 10;
                    let new_state =
                        new_state[0] * 1000 + new_state[1] * 100 + new_state[2] * 10 + new_state[3];

                    if !deadends.contains(&new_state) {
                        deadends.insert(new_state);
                        states.push_back((new_state, i + 1));
                    }
                }
            }
        }

        -1
    }
}
```
