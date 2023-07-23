# 752. Open the Lock
You have a lock in front of you with 4 circular wheels. Each wheel has 10 slots: `'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'`. The wheels can rotate freely and wrap around: for example we can turn `'9'` to be `'0'`, or `'0'` to be `'9'`. Each move consists of turning one wheel one slot.

The lock initially starts at `'0000'`, a string representing the state of the 4 wheels.

You are given a list of `deadends` dead ends, meaning if the lock displays any of these codes, the wheels of the lock will stop turning and you will be unable to open it.

Given a `target` representing the value of the wheels that will unlock the lock, return the minimum total number of turns required to open the lock, or -1 if it is impossible.

#### Example 1:
<pre>
<strong>Input:</strong> deadends = ["0201","0101","0102","1212","2002"], target = "0202"
<strong>Output:</strong> 6
<strong>Explanation:</strong>
A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
because the wheels of the lock become stuck after the display becomes the dead end "0102".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> deadends = ["8888"], target = "0009"
<strong>Output:</strong> 1
<strong>Explanation:</strong>
We can turn the last wheel in reverse to move from "0000" -> "0009".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
<strong>Output:</strong> -1
<strong>Explanation:</strong>
We can't reach the target without getting stuck.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> deadends = ["0000"], target = "8888"
<strong>Output:</strong> -1
</pre>

#### Constraints:
* `1 <= deadends.length <= 500`
* `deadends[i].length == 4`
* `target.length == 4`
* target **will not be** in the list `deadends`.
* `target` and `deadends[i]` consist of digits only.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
