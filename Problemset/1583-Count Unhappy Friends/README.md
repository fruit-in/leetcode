# 1583. Count Unhappy Friends
You are given a list of `preferences` for `n` friends, where `n` is always **even**.

For each person `i`, `preferences[i]` contains a list of friends **sorted** in the **order of preference**. In other words, a friend earlier in the list is more preferred than a friend later in the list. Friends in each list are denoted by integers from `0` to `n-1`.

All the friends are divided into pairs. The pairings are given in a list `pairs`, where <code>pairs[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> denotes <code>x<sub>i</sub></code> is paired with <code>y<sub>i</sub></code> and <code>y<sub>i</sub></code> is paired with <code>x<sub>i</sub></code>.

However, this pairing may cause some of the friends to be unhappy. A friend `x` is unhappy if `x` is paired with `y` and there exists a friend `u` who is paired with `v` but:
* `x` prefers `u` over `y`, and
* `u` prefers `x` over `v`.

Return *the number of unhappy friends*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4, preferences = [[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]], pairs = [[0, 1], [2, 3]]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
Friend 1 is unhappy because:
- 1 is paired with 0 but prefers 3 over 0, and
- 3 prefers 1 over 2.
Friend 3 is unhappy because:
- 3 is paired with 2 but prefers 1 over 2, and
- 1 prefers 3 over 0.
Friends 0 and 2 are happy.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, preferences = [[1], [0]], pairs = [[1, 0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Both friends 0 and 1 are happy.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4, preferences = [[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]], pairs = [[1, 3], [0, 2]]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `2 <= n <= 500`
* `n` is even.
* `preferences.length == n`
* `preferences[i].length == n - 1`
* `0 <= preferences[i][j] <= n - 1`
* `preferences[i]` does not contain `i`.
* All values in `preferences[i]` are unique.
* `pairs.length == n/2`
* `pairs[i].length == 2`
* <code>x<sub>i</sub> != y<sub>i</sub></code>
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1</code>
* Each person is contained in **exactly one** pair.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut rank = vec![vec![n; n]; n];
        let mut is_unhappy = vec![false; n];

        for i in 0..n {
            for j in 0..n - 1 {
                rank[i][preferences[i][j] as usize] = j;
            }
        }

        for i in 0..n / 2 {
            let (x, y) = (pairs[i][0] as usize, pairs[i][1] as usize);

            for j in 0..n / 2 {
                if is_unhappy[x] && is_unhappy[y] {
                    break;
                }

                let (u, v) = (pairs[j][0] as usize, pairs[j][1] as usize);

                is_unhappy[x] |= rank[x][u] < rank[x][y] && rank[u][x] < rank[u][v];
                is_unhappy[x] |= rank[x][v] < rank[x][y] && rank[v][x] < rank[v][u];
                is_unhappy[y] |= rank[y][u] < rank[y][x] && rank[u][y] < rank[u][v];
                is_unhappy[y] |= rank[y][v] < rank[y][x] && rank[v][y] < rank[v][u];
            }
        }

        is_unhappy.iter().filter(|&&unhappy| unhappy).count() as i32
    }
}
```
