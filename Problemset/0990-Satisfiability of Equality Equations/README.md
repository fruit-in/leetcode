# 990. Satisfiability of Equality Equations
You are given an array of strings `equations` that represent relationships between variables where each string `equations[i]` is of length `4` and takes one of two different forms: <code>"x<sub>i</sub>==y<sub>i</sub>"</code> or <code>"x<sub>i</sub>!=y<sub>i</sub>"</code>.Here, <code>x<sub>i</sub></code> and <code>y<sub>i</sub></code> are lowercase letters (not necessarily different) that represent one-letter variable names.

Return `true` *if it is possible to assign integers to variable names so as to satisfy all the given equations, or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> equations = ["a==b","b!=a"]
<strong>Output:</strong> false
<strong>Explanation:</strong> If we assign say, a = 1 and b = 1, then the first equation is satisfied, but not the second.
There is no way to assign the variables to satisfy both equations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> equations = ["b==a","a==b"]
<strong>Output:</strong> true
<strong>Explanation:</strong> We could assign a = 1 and b = 1 to satisfy both equations.
</pre>

#### Constraints:
* `1 <= equations.length <= 500`
* `equations[i].length == 4`
* `equations[i][0]` is a lowercase letter.
* `equations[i][1]` is either `'='` or `'!'`.
* `equations[i][2]` is `'='`.
* `equations[i][3]` is a lowercase letter.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut parent = (0..26).collect::<Vec<usize>>();

        for i in 0..equations.len() {
            let equation = equations[i].as_bytes();
            let mut x = (equation[0] - b'a') as usize;
            let mut y = (equation[3] - b'a') as usize;

            if equation[1] == b'=' {
                while parent[x] != x {
                    x = parent[x];
                }
                while parent[y] != y {
                    y = parent[y];
                }
                if x > y {
                    parent[x] = y;
                } else {
                    parent[y] = x;
                }
            }
        }

        for i in 0..26 {
            while parent[i] != parent[parent[i]] {
                parent[i] = parent[parent[i]];
            }
        }

        for i in 0..equations.len() {
            let equation = equations[i].as_bytes();
            let x = (equation[0] - b'a') as usize;
            let y = (equation[3] - b'a') as usize;

            if equation[1] == b'!' && parent[x] == parent[y] {
                return false;
            }
        }

        true
    }
}
```
