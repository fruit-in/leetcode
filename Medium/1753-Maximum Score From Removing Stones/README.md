# 1753. Maximum Score From Removing Stones
You are playing a solitaire game with **three piles** of stones of sizes `a`, `b`, and `c` respectively. Each turn you choose two **different non-empty** piles, take one stone from each, and add `1` point to your score. The game stops when there are **fewer than two non-empty** piles (meaning there are no more available moves).

Given three integers `a`, `b`, and `c`, return *the **maximum score** you can get*.

#### Example 1:
<pre>
<strong>Input:</strong> a = 2, b = 4, c = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> The starting state is (2, 4, 6). One optimal set of moves is:
- Take from 1st and 3rd piles, state is now (1, 4, 5)
- Take from 1st and 3rd piles, state is now (0, 4, 4)
- Take from 2nd and 3rd piles, state is now (0, 3, 3)
- Take from 2nd and 3rd piles, state is now (0, 2, 2)
- Take from 2nd and 3rd piles, state is now (0, 1, 1)
- Take from 2nd and 3rd piles, state is now (0, 0, 0)
There are fewer than two non-empty piles, so the game ends. Total: 6 points.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = 4, b = 4, c = 6
<strong>Output:</strong> 7
<strong>Explanation:</strong> The starting state is (4, 4, 6). One optimal set of moves is:
- Take from 1st and 2nd piles, state is now (3, 3, 6)
- Take from 1st and 3rd piles, state is now (2, 3, 5)
- Take from 1st and 3rd piles, state is now (1, 3, 4)
- Take from 1st and 3rd piles, state is now (0, 3, 3)
- Take from 2nd and 3rd piles, state is now (0, 2, 2)
- Take from 2nd and 3rd piles, state is now (0, 1, 1)
- Take from 2nd and 3rd piles, state is now (0, 0, 0)
There are fewer than two non-empty piles, so the game ends. Total: 7 points.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> a = 1, b = 8, c = 8
<strong>Output:</strong> 8
<strong>Explanation:</strong> One optimal set of moves is to take from the 2nd and 3rd piles for 8 turns until they are empty.
After that, there are fewer than two non-empty piles, so the game ends.
</pre>

#### Constraints:
* <code>1 <= a, b, c <= 10<sup>5</sup></code>

## Solutions (Ruby)

### 1. Mathematical
```Ruby
# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @return {Integer}
def maximum_score(a, b, c)
  m = [a, b, c].max
  s = [a, b, c].sum

  s <= 2 * m ? s - m : s / 2
end
```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let m = a.max(b).max(c);
        let s = a + b + c;

        if s <= 2 * m {
            s - m
        } else {
            s / 2
        }
    }
}
```
