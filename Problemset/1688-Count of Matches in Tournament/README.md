# 1688. Count of Matches in Tournament
You are given an integer `n`, the number of teams in a tournament that has strange rules:
* If the current number of teams is **even**, each team gets paired with another team. A total of `n / 2` matches are played, and `n / 2` teams advance to the next round.
* If the current number of teams is **odd**, one team randomly advances in the tournament, and the rest gets paired. A total of `(n - 1) / 2` matches are played, and `(n - 1) / 2 + 1` teams advance to the next round.

Return *the number of matches played in the tournament until a winner is decided*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 7
<strong>Output:</strong> 6
<strong>Explanation:</strong> Details of the tournament:
- 1st Round: Teams = 7, Matches = 3, and 4 teams advance.
- 2nd Round: Teams = 4, Matches = 2, and 2 teams advance.
- 3rd Round: Teams = 2, Matches = 1, and 1 team is declared the winner.
Total number of matches = 3 + 2 + 1 = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 14
<strong>Output:</strong> 13
<strong>Explanation:</strong> Details of the tournament:
- 1st Round: Teams = 14, Matches = 7, and 7 teams advance.
- 2nd Round: Teams = 7, Matches = 3, and 4 teams advance.
- 3rd Round: Teams = 4, Matches = 2, and 2 teams advance.
- 4th Round: Teams = 2, Matches = 1, and 1 team is declared the winner.
Total number of matches = 7 + 3 + 2 + 1 = 13.
</pre>

#### Constraints:
* `1 <= n <= 200`

## Solutions (Ruby)

### 1. Simulation
```Ruby
# @param {Integer} n
# @return {Integer}
def number_of_matches(n)
  if n == 1
    0
  elsif n.even?
    n / 2 + number_of_matches(n / 2)
  else
    (n - 1) / 2 + number_of_matches((n - 1) / 2 + 1)
  end
end
```

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        if n == 1 {
            0
        } else if n % 2 == 0 {
            n / 2 + Self::number_of_matches(n / 2)
        } else {
            (n - 1) / 2 + Self::number_of_matches((n - 1) / 2 + 1)
        }
    }
}
```
