# 1575. Count All Possible Routes
You are given an array of **distinct** positive integers locations where `locations[i]` represents the position of city `i`. You are also given integers `start`, `finish` and `fuel` representing the starting city, ending city, and the initial amount of fuel you have, respectively.

At each step, if you are at city `i`, you can pick any city `j` such that `j != i` and `0 <= j < locations.length` and move to city `j`. Moving from city `i` to city `j` reduces the amount of fuel you have by `|locations[i] - locations[j]|`. Please notice that `|x|` denotes the absolute value of x.

Notice that `fuel` **cannot** become negative at any point in time, and that you are **allowed** to visit any city more than once (including `start` and `finish`).

Return *the count of all possible routes from* `start` *to* `finish`. Since the answer may be too large, return it modulo <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> locations = [2,3,6,8,4], start = 1, finish = 3, fuel = 5
<strong>Output:</strong> 4
<strong>Explanation:</strong> The following are all possible routes, each uses 5 units of fuel:
1 -> 3
1 -> 2 -> 3
1 -> 4 -> 3
1 -> 4 -> 2 -> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> locations = [4,3,1], start = 1, finish = 0, fuel = 6
<strong>Output:</strong> 5
<strong>Explanation:</strong> The following are all possible routes:
1 -> 0, used fuel = 1
1 -> 2 -> 0, used fuel = 5
1 -> 2 -> 1 -> 0, used fuel = 5
1 -> 0 -> 1 -> 0, used fuel = 3
1 -> 0 -> 1 -> 0 -> 1 -> 0, used fuel = 5
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> locations = [5,2,1], start = 0, finish = 2, fuel = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> It is impossible to get from 0 to 2 using only 3 units of fuel since the shortest route needs 4 units of fuel.
</pre>

#### Constraints:
* `2 <= locations.length <= 100`
* <code>1 <= locations[i] <= 10<sup>9</sup></code>
* All integers in `locations` are **distinct**.
* `0 <= start, finish < locations.length`
* `1 <= fuel <= 200`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let (start, finish) = (start as usize, finish as usize);
        let fuel = fuel as usize;
        let mut dp = vec![vec![0; fuel + 1]; locations.len()];
        dp[start][0] = 1;

        for i in 0..fuel {
            for j in 0..locations.len() {
                for k in 0..locations.len() {
                    let cost = (locations[j] - locations[k]).abs() as usize;

                    if j != k && i + cost <= fuel {
                        dp[k][i + cost] = (dp[k][i + cost] + dp[j][i]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[finish]
            .iter()
            .fold(0, |acc, x| (acc + x) % 1_000_000_007)
    }
}
```
